package com.demo.presto.hook;

import com.facebook.presto.spi.eventlistener.EventListener;
import com.facebook.presto.spi.eventlistener.EventListenerFactory;

import java.util.Map;

public class PrestoHookFactory implements EventListenerFactory {
    @Override
    public String getName() {
        return "presto-hook";
    }

    @Override
    public EventListener create(Map<String, String> config) {
        return new PrestoHook(config);
    }
}
