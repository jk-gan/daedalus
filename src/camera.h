//
// Created by Jun Kai Gan on 01/08/2024.
//

#pragma once

#include <glm/glm.hpp>


class camera {
public:
    double aspect_ratio = 1.0; // ratio of image width over height
    int image_width = 100; // rendered image width in pixel count
    int samples_per_pixel = 10; // count of random samples for each pixel
    int max_depth = 10; // maximum number of ray bounces into scene

    double vfov = 90.0; // vertical view angle (field of view) in degrees
    glm::vec3 look_from = glm::vec3 { 0.0, 0.0, 0.0 }; // point the camera is looking from
    glm::vec3 look_at = glm::vec3 { 0.0, 0.0, -1.0 }; // point the camera is looking at
    glm::vec3 vup = glm::vec3 { 0.0, 1.0, 0.0 }; // camera-relative up direction

    double defocus_angle = 0.0; // variation angle of rays through each pixel
    double focus_distance = 10.0; // distance from camera look_from point to plane of perfect focus

    // auto render(const hittable& world) -> void {
    //     initialize();
    //
    //     std::cout << "P3\n" << image_width << " " << image_height << "\n255\n";
    //
    //     for (int j = 0; j < image_height; j++) {
    //         std::clog << "\rScanlines remaining: " << (image_height - j) << ' ' << std::flush;
    //         for (int i = 0; i < image_width; i++) {
    //             color pixel_color { 0.0, 0.0, 0.0 };
    //             for (int sample = 0; sample < samples_per_pixel; sample++) {
    //                 ray r = get_ray(i, j);
    //                 pixel_color += ray_color(r, max_depth, world);
    //             }
    //             write_color(std::cout, pixel_samples_scale * pixel_color);
    //         }
    //     }
    //
    //     std::clog << "\rDone.                 \n";
    // }

private:
    int image_height = 0; // rendered image height
    double pixel_samples_scale; // color scale factor for a sum of pixel samples
    glm::vec3 center; // camera center
    glm::vec3 pixel00_loc; // location of pixel 0, 0
    glm::vec3 pixel_delta_u; // offset to pixel to the right
    glm::vec3 pixel_delta_v; // offset to pixel to the bottom
    glm::vec3 u, v, w; // camera frame basis vectors
    glm::vec3 defocus_disk_u; // defocus disk horizontal radius
    glm::vec3 defocus_disk_v; // defocus disk vertical radius

    auto initialize() -> void {
        // Calculate the image height, and ensure that it's at least 1
        image_height = static_cast<int>(image_width / aspect_ratio);
        image_height = (image_height < 1) ? 1 : image_height;

        pixel_samples_scale = 1.0 / samples_per_pixel;

        center = look_from;

        // Determine viewport dimensions
        const auto theta = degrees_to_radians(vfov);
        const auto h = std::tan(theta / 2.0);
        const auto viewport_height = 2.0 * h * focus_distance;
        const auto viewport_width = viewport_height * (static_cast<double>(image_width) / image_height);

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame
        w = unit_vector(look_from - look_at);
        u = unit_vector(cross(vup, w));
        v = cross(w, u);

        // Calculate the vector across the horizontal (vector u) and down the vertical viewport edges (vector v)
        const auto viewport_u = viewport_width * u; // vector across viewport horizontal edge
        const auto viewport_v = viewport_height * -v; // vector down viewport vertical edge

        // Calculate the horizontal (delta u) and vertical (delta v) delta vectors from pixel to pixel
        pixel_delta_u = viewport_u / image_width;
        pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left corner of the viewport
        const auto viewport_upper_left = center - (focus_distance * w) - viewport_u / 2 - viewport_v / 2;
        pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors
        auto defocus_radius = focus_distance * std::tan(degrees_to_radians(defocus_angle / 2.0));
        defocus_disk_u = u * defocus_radius;
        defocus_disk_v = v * defocus_radius;
    }

    [[nodiscard]] auto get_ray(int i, int j) const -> ray {
        // Construct a camera ray originating from the defocus disk and directed at randomly
        // sampled point around the pixel location (i, j)
        const auto offset = sample_square();
        const auto pixel_sample = pixel00_loc + ((i + offset.x()) * pixel_delta_u) + ((j + offset.y()) * pixel_delta_v);

        const auto ray_origin = (defocus_angle <= 0.0) ? center : defocus_disk_sample();
        const auto ray_direction = pixel_sample - ray_origin;

        return ray { ray_origin, ray_direction };
    }

    [[nodiscard]] auto sample_square() const -> vec3 {
        // Returns the vector to a random point in the [-0.5, -0.5] - [+0.5, +0.5] unit square
        return vec3 { random_double() - 0.5, random_double() - 0.5, 0.0 };
    }

    auto defocus_disk_sample() const -> point3 {
        // Returns a random point in the camera defocus disk
        auto p = random_in_unit_disk();
        return center + (p[0] * defocus_disk_u) + (p[1] * defocus_disk_v);
    }

    auto ray_color(const ray& r, int depth, const hittable& world) -> color {
        // if we've exceeded the ray bounce limit, no more light is gathered
        if (depth <= 0)
            return color { 0.0, 0.0, 0.0 };

        hit_record rec;

        if (world.hit(r, interval { 0.001, DOUBLE_INFINITY }, rec)) {
            ray scattered;
            color attenuation;
            if (rec.material->scatter(r, rec, attenuation, scattered)) {
                return attenuation * ray_color(scattered, depth - 1, world);
            }
            return color { 0.0, 0.0, 0.0 };
        }

        vec3 unit_direction = unit_vector(r.direction());
        auto a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * color { 1.0, 1.0, 1.0 } + a * color { 0.5, 0.7, 1.0 };
    }
};
