//
// Created by Jun Kai Gan on 23/07/2024.
//

#pragma once
#include <AppKit/AppKit.hpp>

#include "mtk_view_delegate.h"

class AppDelegate : public NS::ApplicationDelegate {
public:
    ~AppDelegate();
    auto createMenuBar() -> NS::Menu*;
    virtual auto applicationWillFinishLaunching(NS::Notification* pNotification) -> void override;
    virtual auto applicationDidFinishLaunching(NS::Notification* pNotification) -> void override;
    virtual auto applicationShouldTerminateAfterLastWindowClosed(NS::Application* pSender) -> bool override;

private:
    NS::Window* window;
    MTK::View* view;
    MTL::Device* device;
    MTKViewDelegate* viewDelegate = nullptr;
    ;
};
