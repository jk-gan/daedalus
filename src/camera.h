// //
// // Created by Jun Kai Gan on 01/08/2024.
// //

#pragma once

#include "shader_types.h"

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <simd/simd.h>

class Camera {
public:
    Camera(const glm::vec3& origin)
        : origin(origin) { }
    Camera(glm::vec3 origin, glm::vec3 u, glm::vec3 v, glm::vec3 w)
        : origin(origin)
        , u(u)
        , v(v)
        , w(w) { }

    auto static look_at(const glm::vec3 origin, const glm::vec3 center, const glm::vec3 up) -> std::unique_ptr<Camera> {
        auto w = glm::normalize(center - origin);
        auto u = glm::normalize(glm::cross(up, w));
        auto v = glm::cross(w, u);

        return std::make_unique<Camera>(origin, u, v, w);
    }

    auto zoom(float displacement) -> void { origin += displacement * w; }
    auto move_forward(float amount) -> void { origin += amount * w; }
    auto move_right(float amount) -> void { origin += amount * u; }
    auto move_up(float amount) -> void { origin += amount * v; }

    auto rotate_horizontal(float angle) -> void {
        glm::mat4 rotation = glm::rotate(glm::mat4(1.0f), angle, v);
        u = glm::vec3(rotation * glm::vec4(u, 0.0f));
        w = glm::vec3(rotation * glm::vec4(w, 0.0f));
    }

    auto rotate_vertical(float angle) -> void {
        glm::mat4 rotation = glm::rotate(glm::mat4(1.0f), angle, u);
        v = glm::vec3(rotation * glm::vec4(v, 0.0f));
        w = glm::vec3(rotation * glm::vec4(w, 0.0f));
    }

    [[nodiscard]] auto get_uniforms() const -> const Shader_CameraUniforms {
        return Shader_CameraUniforms {
            .origin = reinterpret_cast<const vector_float3&>(origin),
            .u = reinterpret_cast<const vector_float3&>(u),
            .v = reinterpret_cast<const vector_float3&>(v),
            .w = reinterpret_cast<const vector_float3&>(w),
        };
    }

private:
    glm::vec3 origin;
    glm::vec3 u;
    glm::vec3 v;
    glm::vec3 w;

    float speed = 5.0f;
};
