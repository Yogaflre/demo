package sql.handler;

import metadata.ResourceReader;
import org.apache.calcite.adapter.jdbc.JdbcSchema;
import org.apache.calcite.rel.RelNode;
import org.apache.calcite.rel.rel2sql.RelToSqlConverter;
import org.apache.calcite.schema.SchemaPlus;
import org.apache.calcite.sql.SqlDialect;
import org.apache.calcite.sql.SqlNode;
import org.apache.calcite.sql.dialect.MysqlSqlDialect;
import sql.enums.SqlResultType;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;

public class PrestoHandler implements SqlHandler {
    @Override
    public SchemaPlus createSchema(SchemaPlus rootSchema, Set<String> tableNames) throws IOException {
        Map<String, Object> prestoConfig = ResourceReader.getJdbcResources(SqlResultType.PRESTO);
        if (!tableNames.isEmpty()) {
            for (String tableName : tableNames) {
                Map<String, Object> configMap = new HashMap<>(prestoConfig);
                String schemaName = tableName.split("\\.", 2)[0];
                configMap.put("jdbcSchema", schemaName);
                JdbcSchema jdbcSchema = JdbcSchema.create(rootSchema, schemaName, configMap);
                rootSchema.add(schemaName, jdbcSchema);
            }
        } else {
            JdbcSchema jdbcSchema = JdbcSchema.create(rootSchema, null, prestoConfig);
            rootSchema.add(null, jdbcSchema);
        }
        return rootSchema;
    }

    @Override
    public String createSql(RelNode relNode) {
        MysqlSqlDialect dialect = new MysqlSqlDialect(SqlDialect.EMPTY_CONTEXT);
        SqlNode sqlNode = new RelToSqlConverter(dialect).visitChild(0, relNode).asStatement();
        return sqlNode.toSqlString(dialect).getSql();
    }
}
