//
// Created by Jun Kai Gan on 04/08/2024.
//

#pragma once

#include "shader_types.h"

#include <SDL3/SDL.h>
#include <glm/glm.hpp>
#include <simd/simd.h>
#include <spdlog/spdlog.h>

#include "glm/ext/matrix_clip_space.hpp"
#include "glm/ext/matrix_transform.hpp"

class FirstPersonCamera {
public:
    FirstPersonCamera(glm::vec3 position, float yaw_degrees, float pitch_degrees, float fov_degrees, float aspect_ratio,
        float z_far, float z_near)
        : position(position)
        , yaw_radians(glm::radians(yaw_degrees))
        , pitch_radians(glm::radians(pitch_degrees))
        , fov_radians(glm::radians(fov_degrees))
        , aspect_ratio(aspect_ratio)
        , z_far(z_far)
        , z_near(z_near) { }

    auto set_aspect_ratio(float aspect_ratio) -> void { this->aspect_ratio = aspect_ratio; }

    auto view_matrix() const -> glm::mat4 {
        float sin_pitch = std::sin(pitch_radians);
        float cos_pitch = std::cos(pitch_radians);
        float sin_yaw = std::sin(yaw_radians);
        float cos_yaw = std::cos(yaw_radians);

        glm::vec3 front(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw);

        return glm::lookAtRH(position, position + glm::normalize(front), glm::vec3(0.0f, 1.0f, 0.0f));
    }

    auto projection_matrix() const -> glm::mat4 { return glm::perspectiveRH(fov_radians, aspect_ratio, z_near, z_far); }

    auto get_w() const -> glm::vec3 { return -get_front(); }
    auto get_u() const -> glm::vec3 { return get_right(); }
    auto get_v() const -> glm::vec3 { return get_up(); }

    [[nodiscard]] auto get_uniforms() const -> const Shader_CameraUniforms {
        auto u = get_u();
        auto v = get_v();
        auto w = get_w();
        return Shader_CameraUniforms {
            .origin = reinterpret_cast<const vector_float3&>(position),
            .u = reinterpret_cast<const vector_float3&>(u),
            .v = reinterpret_cast<const vector_float3&>(v),
            .w = reinterpret_cast<const vector_float3&>(w),
        };
    }

    glm::vec3 position;
    float yaw_radians;
    float pitch_radians;

private:
    float fov_radians;
    float aspect_ratio;
    float z_far;
    float z_near;

    auto get_front() const -> glm::vec3 {
        return glm::vec3 {
            glm::cos(pitch_radians) * glm::cos(yaw_radians),
            glm::sin(pitch_radians),
            glm::cos(pitch_radians) * glm::sin(yaw_radians),
        };
    }
    auto get_right() const -> glm::vec3 { return glm::normalize(glm::cross(get_front(), glm::vec3(0.0f, 1.0f, 0.0f))); }
    auto get_up() const -> glm::vec3 { return glm::normalize(glm::cross(get_right(), get_front())); }
};

class CameraController {
public:
    CameraController(float speed, float sensitivity)
        : speed(speed)
        , sensitivity(sensitivity) { }

    auto handle_keyboard_event(SDL_Keycode key, SDL_EventType type) -> void {
        float amount = (type == SDL_EVENT_KEY_DOWN) ? 1.0f : 0.0f;
        spdlog::info("Key {} amount {}", key, amount);

        switch (key) {
            case SDLK_W:
            case SDLK_UP:
                amount_forward = amount;
                break;
            case SDLK_S:
            case SDLK_DOWN:
                amount_backward = amount;
                break;
            case SDLK_A:
            case SDLK_LEFT:
                amount_left = amount;
                break;
            case SDLK_D:
            case SDLK_RIGHT:
                amount_right = amount;
                break;
            case SDLK_SPACE:
                amount_up = amount;
                break;
            case SDLK_LSHIFT:
                amount_down = amount;
                break;
        }
    }

    auto handle_mouse_event(float mouse_dx, float mouse_dy) -> void {
        rotate_horizontal = mouse_dx;
        rotate_vertical = mouse_dy;
    }

    auto handle_scroll_event(float scroll_y) -> void { scroll = -scroll_y * 0.5f; }

    auto update(FirstPersonCamera& camera, float delta_time) -> void {
        // Move forward/backward and left/right
        float yaw_sin = std::sin(camera.yaw_radians);
        float yaw_cos = std::cos(camera.yaw_radians);
        glm::vec3 forward { yaw_cos, 0.0f, yaw_sin };
        glm::vec3 right { -yaw_sin, 0.0f, yaw_cos };

        camera.position += glm::normalize(forward) * (amount_forward - amount_backward) * speed * delta_time;
        camera.position += glm::normalize(right) * (amount_right - amount_left) * speed * delta_time;

        // Move in/out (zoom)
        float pitch_sin = std::sin(camera.pitch_radians);
        float pitch_cos = std::cos(camera.pitch_radians);
        glm::vec3 scrollward { pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin };
        camera.position += glm::normalize(scrollward) * scroll * speed * sensitivity * delta_time;
        scroll = 0.0f;

        // Move up/down
        camera.position.y += (amount_up - amount_down) * speed * delta_time;

        // Rotate
        camera.yaw_radians += glm::radians(rotate_horizontal) * sensitivity * delta_time;
        camera.pitch_radians += glm::radians(-rotate_vertical) * sensitivity * delta_time;

        // Reset rotation values
        rotate_horizontal = 0.0f;
        rotate_vertical = 0.0f;

        // Clamp pitch
        camera.pitch_radians = glm::clamp(camera.pitch_radians, glm::radians(-89.0f), glm::radians(89.0f));
    }

private:
    float amount_left = 0.0f, amount_right = 0.0f;
    float amount_forward = 0.0f, amount_backward = 0.0f;
    float amount_up = 0.0f, amount_down = 0.0f;
    float rotate_horizontal = 0.0f, rotate_vertical = 0.0f;
    float scroll = 0.0f;
    float speed;
    float sensitivity;
};
