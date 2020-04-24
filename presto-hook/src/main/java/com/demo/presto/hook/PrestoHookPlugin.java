package com.demo.presto.hook;

import com.facebook.presto.spi.Plugin;
import com.facebook.presto.spi.eventlistener.EventListenerFactory;

import java.util.Collections;

public class PrestoHookPlugin implements Plugin {
    @Override
    public Iterable<EventListenerFactory> getEventListenerFactories() {
        return Collections.singletonList(new com.demo.presto.hook.PrestoHookFactory());
    }
}
