package com.demo.http_example_java;

import java.io.IOException;
import java.io.InputStream;
import java.util.Properties;

/**
 * Created by yogafire on 2020/10/15
 */
public enum HttpConfig {
    //所有连接最大连接数
    MAX_CONNECTIONS("max_connections", "100"),
    //每个服务默认最大连接数
    MAX_ROUTE_CONNECTIONS("max_route_connections", "20"),
    //http keep-alive时间
    KEEP_ALIVE_DURATION("keep_alive_duration", "10000"),
    //从连接池获取连接超时时间
    CONNECTION_REQUEST_TIMEOUT("connection_request_timeout", "5000"),
    //连接超时时间
    CONNECTION_TIMEOUT("connection_timeout", "5000"),
    //socket读取等待时间
    SOCKET_READ_TIMEOUT("socket_read_timeout", "10000"),
    //校验不活跃的连接。如果活跃时间+该配置 < 当前时间，则进行校验连接是否正常(该配置小于keep-alive时间)
    VALIDATE_AFTER_INACTIVITY("validate_after_inactivity", "8000"),
    ;

    public String key;
    public String value;


    HttpConfig(String key, String value) {
        this.key = key;
        this.value = value;
    }

    static {
        try {
            InputStream stream = ClassLoader.getSystemResourceAsStream("http.properties");
            if (stream != null) {
                Properties prop = new Properties();
                prop.load(stream);
                for (HttpConfig config : HttpConfig.values()) {
                    setDefaultValue(prop, config);
                }
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private static void setDefaultValue(Properties prop, HttpConfig config) {
        String value = prop.getProperty(config.key);
        if (value != null && !value.isEmpty()) {
            config.value = value;
        }
    }

    public long longValue() {
        return Long.parseLong(this.value);
    }

    public int intValue() {
        return Integer.parseInt(this.value);
    }

}
