#include <metal_stdlib>
#include "definitions.h"

using namespace metal;

constant float FLOAT_MAX = 3.40282346638528859812e+38;
constant uint OBJECT_COUNT = 2;
constant uint MAX_PATH_LENGTH = 13;
constant float EPSILON = 1e-3;

struct Rng {
    uint state;
};

// A slightly modified version of the "One-at-a-Time Hash" function by Bob Jenkins.
uint jenkins_hash(uint i) {
    uint x = i;
    x += x << 10u;
    x ^= x >> 6u;
    x += x << 3u;
    x ^= x >> 11u;
    x += x << 15u;
    return x;
}

// Initialize RNG
Rng init_rng(uint2 pixel, constant Shader_Uniforms& uniforms) {
    // Seed the PRNG using the scalar index of the pixel and the current frame count.
    uint seed = (pixel.x + pixel.y * uniforms.width) ^ jenkins_hash(uniforms.frame_count);
    return Rng { jenkins_hash(seed) };
}

// The 32-bit "xor" function from Marsaglia G., "Xorshift RNGs", Section 3.
uint xorshift32(thread Rng& rng) {
    uint x = rng.state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    rng.state = x;
    return x;
}

// Returns a random float in the range [0...1].
float rand_f32(thread Rng& rng) {
    return as_type<float>(0x3f800000u | (xorshift32(rng) >> 9u)) - 1.0;
}

struct VertexOut {
    float4 position [[position]];
};

constant float2 vertices[6] = {
    float2(-1.0, 1.0),
    float2(-1.0, -1.0),
    float2(1.0, 1.0),
    float2(1.0, 1.0),
    float2(-1.0, -1.0),
    float2(1.0, -1.0),
};

constant Shader_Sphere spheres[OBJECT_COUNT] = {
    Shader_Sphere { float3 { 0.0, 0.0, -1.0 }, 0.5, float3 { 0.5, 0.4, 0.0 } },
    Shader_Sphere { float3 { 0.0, -100.5, -1.0 }, 100.0, float3 { 0.7, 0.4, 0.6 } },
};

vertex VertexOut vertex_main(uint vertexID [[ vertex_id ]]) {
    VertexOut out;
    out.position = float4(vertices[vertexID], 0.0, 1.0);
    return out;
}

float3 point_on_ray(Shader_Ray ray, float t);

Shader_Intersection no_intersection() {
    return Shader_Intersection { vector_float3 { 0.0, 0.0, 0.0 }, -1.0, float3(0.0) };
}

bool is_intersection_valid(Shader_Intersection intersection) {
    return intersection.t > 0.0;
}

Shader_Intersection intersect_sphere(Shader_Ray ray, Shader_Sphere sphere) {
    auto v = ray.origin - sphere.center;
    auto a = dot(ray.direction, ray.direction);
    auto b = dot(v, ray.direction);
    auto c = dot(v, v) - sphere.radius * sphere.radius;

    auto discriminant = b * b - a * c;
    if (discriminant < 0.0) {
        return no_intersection();
    }

    auto sqrt_discriminant = sqrt(discriminant);
    auto recip_a = 1.0 / a;
    auto mb = -b;
    auto t1 = (mb - sqrt_discriminant) * recip_a;
    auto t2 = (mb + sqrt_discriminant) * recip_a;
    auto t = select(t2, t1, t1 > EPSILON);
    if (t <= EPSILON) {
        return no_intersection();
    }

    auto p = point_on_ray(ray, t);
    auto N = (p - sphere.center) / sphere.radius;
    return Shader_Intersection { N, t, sphere.color };
}

Shader_Scatter scatter(Shader_Ray input_ray, Shader_Intersection hit) {
    auto reflected = reflect(input_ray.direction, hit.normal);
    auto output_ray = Shader_Ray { point_on_ray(input_ray, hit.t), reflected };
    auto attenuation = hit.color;
    return Shader_Scatter { attenuation, output_ray };
}

float3 point_on_ray(Shader_Ray ray, float t) {
    return ray.origin + t * ray.direction;
}

float3 sky_color(Shader_Ray ray) {
    auto t = 0.5 * (normalize(ray.direction).y + 1.0);
    return (1.0 - t) * float3(1.0, 1.0, 1.0) + t * float3(0.3, 0.5, 1.0);
}

Shader_Intersection intersect_scene(Shader_Ray ray) {
    auto closest_hit = no_intersection();
    closest_hit.t = FLOAT_MAX;
    for (uint i = 0; i < OBJECT_COUNT; i++) {
        auto sphere = spheres[i];
        auto hit = intersect_sphere(ray, sphere);
        if (hit.t > 0.0 && hit.t < closest_hit.t) {
            closest_hit = hit;
        }
    }
    if (closest_hit.t < FLOAT_MAX) {
        return closest_hit;
    }
    return no_intersection();
}

fragment float4 fragment_main(float4 position [[ position ]], constant Shader_Uniforms& uniforms [[ buffer(0) ]],
                              texture2d<float> radiance_sample_old [[ texture(0) ]],
                              texture2d<float, access::write> radiance_sample_new [[ texture(1) ]]
) {
    Rng rng = init_rng(uint2(position.xy), uniforms);
    float3 origin = uniforms.camera.origin;
    float focus_distance = 1.0;
    float aspect_ratio = static_cast<float>(uniforms.width) / static_cast<float>(uniforms.height);

    auto offset = float2(rand_f32(rng) - 0.5, rand_f32(rng) - 0.5);
    // Normalize the viewport coordinates
    auto uv = (position.xy + offset) / float2(static_cast<float>(uniforms.width - 1), static_cast<float>(uniforms.height - 1));

    // Map `uv` from y-dowm (normalized) viewport coordinated to camera coordinates
    uv = (2.0 * uv - float2(1.0)) * float2(aspect_ratio, -1.0);

    // Compute the scene-space ray direction by rotating the camera-space vector into a new basis
    auto camera_rotation = float3x3(uniforms.camera.u, uniforms.camera.v, uniforms.camera.w);
    float3 direction = camera_rotation * float3(uv, focus_distance);
    Shader_Ray ray { origin, direction };
    auto throughput = float3(1.0);
    auto radiance_sample = float3(0.0);

    auto path_length = 0;
    while (path_length < MAX_PATH_LENGTH) {
        auto hit = intersect_scene(ray);
        if (!is_intersection_valid(hit)) {
            // If no intersection is found, return the color of the sky and terminate the path.
            radiance_sample += throughput * sky_color(ray);
            break;
        }
        auto scattered = scatter(ray, hit);
        throughput *= scattered.attenuation;
        ray = scattered.ray;
        path_length++;
    }

    // Fetch the old sum of samples
    float3 old_sum;
    if (uniforms.frame_count > 1) {
        old_sum = radiance_sample_old.read(uint2(position.xy)).rgb;
    } else {
        old_sum = float3(0.0);
    }

    // Compute and store the new sum
    float3 new_sum = radiance_sample + old_sum;
    radiance_sample_new.write(float4(new_sum, 1.0), uint2(position.xy));

    // Display the average
    auto color = new_sum / static_cast<float>(uniforms.frame_count);
    return float4(pow(color, float3(1.0 / 2.2)), 1.0);
}