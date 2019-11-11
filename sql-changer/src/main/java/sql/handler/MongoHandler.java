package sql.handler;

import exception.ParseException;
import metadata.ResourceReader;
import org.apache.calcite.adapter.mongodb.MongoSchemaFactory;
import org.apache.calcite.rel.RelNode;
import org.apache.calcite.schema.Schema;
import org.apache.calcite.schema.SchemaPlus;
import sql.enums.SqlResultType;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;

public class MongoHandler implements SqlHandler {
    @Override
    public SchemaPlus createSchema(SchemaPlus rootSchema, Set<String> tableNames) throws IOException {
        Map<String, Object> esConfig = ResourceReader.getJdbcResources(SqlResultType.MONGO);
        if (!tableNames.isEmpty()) {
            for (String tableName : tableNames) {
                Map<String, Object> configMap = new HashMap<>(esConfig);
                String[] split = tableName.split("\\.", 2);
                configMap.put("database", split[0]);
                configMap.put("authDatabase", split[0]);
                Schema schema = new MongoSchemaFactory().create(rootSchema, null, configMap);
                rootSchema.add(split[0], schema);
            }
        } else {
            throw new ParseException("mongo database is null!");
        }
        return rootSchema;
    }

    @Override
    public String createSql(RelNode relNode) {
        return null;
    }
}
