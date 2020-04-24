package com.demo.presto.hook;


import com.facebook.presto.spi.eventlistener.EventListener;
import com.facebook.presto.spi.eventlistener.QueryCompletedEvent;

import java.util.Map;

public class PrestoHook implements EventListener {

    public PrestoHook(Map<String, String> config) {
    }

    @Override
    public void queryCompleted(QueryCompletedEvent event) {
        // TODO something.
    }
}
