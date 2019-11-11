package sql.handler;

import metadata.ResourceReader;
import org.apache.calcite.adapter.jdbc.JdbcConvention;
import org.apache.calcite.adapter.jdbc.JdbcSchema;
import org.apache.calcite.linq4j.tree.Expression;
import org.apache.calcite.rel.RelNode;
import org.apache.calcite.rel.rel2sql.RelToSqlConverter;
import org.apache.calcite.schema.SchemaPlus;
import org.apache.calcite.schema.Schemas;
import org.apache.calcite.sql.SqlDialect;
import org.apache.calcite.sql.SqlNode;
import org.apache.calcite.sql.dialect.HiveSqlDialect;
import org.apache.commons.dbcp.BasicDataSource;
import sql.enums.SqlResultType;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;

public class HiveHandler implements SqlHandler {
    @Override
    public SchemaPlus createSchema(SchemaPlus rootSchema, Set<String> tableNames) throws IOException {
        Map<String, Object> hiveConfig = ResourceReader.getJdbcResources(SqlResultType.HIVE);
        if (!tableNames.isEmpty()) {
            for (String tableName : tableNames) {
                Map<String, Object> configMap = new HashMap<>(hiveConfig);
                String schemaName = tableName.split("\\.")[0];
                configMap.put("jdbcSchema", schemaName);
                BasicDataSource dataSource = new BasicDataSource();
                dataSource.setDriverClassName(configMap.get("jdbcDriver").toString());
                dataSource.setUrl(configMap.get("jdbcUrl").toString());
                dataSource.setUsername(configMap.get("jdbcUser").toString());

                final Expression expression =
                        Schemas.subSchemaExpression(rootSchema, schemaName, JdbcSchema.class);
                HiveSqlDialect dialect = new HiveSqlDialect(SqlDialect.EMPTY_CONTEXT);
                final JdbcConvention convention = JdbcConvention.of(dialect, expression, schemaName);
                JdbcSchema jdbcSchema = new JdbcSchema(dataSource, new HiveSqlDialect(SqlDialect.EMPTY_CONTEXT), convention, null, schemaName);

//                JdbcSchema jdbcSchema = JdbcSchema.create(rootSchema, schemaName, configMap);
                rootSchema.add(schemaName, jdbcSchema);
            }
        } else {
            JdbcSchema jdbcSchema = JdbcSchema.create(rootSchema, null, hiveConfig);
            rootSchema.add("", jdbcSchema);
        }
        return rootSchema;
    }

    @Override
    public String createSql(RelNode relNode) {
        HiveSqlDialect dialect = new HiveSqlDialect(SqlDialect.EMPTY_CONTEXT);
        SqlNode sqlNode = new RelToSqlConverter(dialect).visitChild(0, relNode).asStatement();
        return sqlNode.toSqlString(dialect).getSql();
    }
}
