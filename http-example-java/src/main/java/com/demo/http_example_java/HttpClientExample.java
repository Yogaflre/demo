package com.demo.http_example_java;

import com.demo.http_example_java.HttpConfig;
import org.apache.http.HttpResponse;
import org.apache.http.client.config.RequestConfig;
import org.apache.http.config.ConnectionConfig;
import org.apache.http.config.SocketConfig;
import org.apache.http.conn.ConnectionKeepAliveStrategy;
import org.apache.http.conn.HttpClientConnectionManager;
import org.apache.http.impl.client.CloseableHttpClient;
import org.apache.http.impl.client.DefaultConnectionKeepAliveStrategy;
import org.apache.http.impl.client.HttpClientBuilder;
import org.apache.http.impl.client.HttpClients;
import org.apache.http.impl.conn.PoolingHttpClientConnectionManager;
import org.apache.http.protocol.HttpContext;

/**
 * Created by yogafire on 2020/10/15
 */
public class HttpClientExample {

    public CloseableHttpClient build() {
        return clientBuilder().build();
    }

    public HttpClientBuilder clientBuilder() {
        return HttpClients.custom()
                .setDefaultRequestConfig(customRequestConfig())
                .setConnectionManager(customConnectionManager())
                .setKeepAliveStrategy(customKeepAliveStrategy());
    }

    /**
     * 自定义请求配置
     */
    private RequestConfig customRequestConfig() {
        return RequestConfig.custom()
                .setConnectionRequestTimeout(HttpConfig.CONNECTION_REQUEST_TIMEOUT.intValue())
                .setConnectTimeout(HttpConfig.CONNECTION_TIMEOUT.intValue())
                .setSocketTimeout(HttpConfig.SOCKET_READ_TIMEOUT.intValue())
                .build();
    }

    /**
     * 自定义连接管理(默认使用连接池)
     */
    private HttpClientConnectionManager customConnectionManager() {
        PoolingHttpClientConnectionManager manager = new PoolingHttpClientConnectionManager();
        manager.setDefaultSocketConfig(customSocketConfig());
        manager.setDefaultConnectionConfig(customConnectionConfig());
        manager.setDefaultMaxPerRoute(HttpConfig.MAX_ROUTE_CONNECTIONS.intValue());
        manager.setMaxTotal(HttpConfig.MAX_CONNECTIONS.intValue());
        manager.setValidateAfterInactivity(HttpConfig.VALIDATE_AFTER_INACTIVITY.intValue());
        return manager;
    }

    /**
     * 自定义socket配置
     */
    private SocketConfig customSocketConfig() {
        return SocketConfig.DEFAULT;
    }

    /**
     * 自定义连接配置
     */
    private ConnectionConfig customConnectionConfig() {
        return ConnectionConfig.DEFAULT;
    }

    /**
     * 自定义http keep-alive策略
     */
    private ConnectionKeepAliveStrategy customKeepAliveStrategy() {
        return new DefaultConnectionKeepAliveStrategy() {
            @Override
            public long getKeepAliveDuration(HttpResponse response, HttpContext context) {
                long keepAlive = super.getKeepAliveDuration(response, context);
                if (keepAlive == -1) {
                    keepAlive = HttpConfig.KEEP_ALIVE_DURATION.longValue();
                }
                return keepAlive;
            }
        };
    }
}
