package metadata;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.JavaType;
import com.fasterxml.jackson.dataformat.yaml.YAMLMapper;
import com.google.common.base.Preconditions;
import sql.enums.SqlResultType;

import java.io.IOException;
import java.io.InputStream;
import java.util.HashMap;
import java.util.Map;

public class ResourceReader {

    private static Map<String, Map<String, Object>> JDBC_CONFIG;

    public static Map<String, Object> getJdbcResources(SqlResultType sqlResultType) throws IOException {
        if (JDBC_CONFIG == null || JDBC_CONFIG.isEmpty()) {
            YAMLMapper yamlMapper = new YAMLMapper();
            try (InputStream resourceStream = ResourceReader.class.getClassLoader().getResourceAsStream("metadata-jdbc.yml")) {
                JavaType javaType = yamlMapper.getTypeFactory().constructType(new TypeReference<HashMap<String, HashMap<String, String>>>() {
                });
                Preconditions.checkNotNull(resourceStream, "Resource InputStream is null!");
                JDBC_CONFIG = yamlMapper.readValue(resourceStream, javaType);
            }
        }
        return JDBC_CONFIG.get(sqlResultType.name());
    }

}
