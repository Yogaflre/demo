package sql.handler;

import org.apache.calcite.rel.RelNode;
import org.apache.calcite.rel.rel2sql.RelToSqlConverter;
import org.apache.calcite.schema.SchemaPlus;
import org.apache.calcite.sql.SqlDialect;
import org.apache.calcite.sql.SqlNode;
import org.apache.calcite.sql.dialect.PhoenixSqlDialect;

import java.io.IOException;
import java.util.Set;

public class PhoenixHandler implements SqlHandler {
    @Override
    public SchemaPlus createSchema(SchemaPlus rootSchema, Set<String> tableNames) throws IOException {
        return null;
    }

    @Override
    public String createSql(RelNode relNode) {
        PhoenixSqlDialect dialect = new PhoenixSqlDialect(SqlDialect.EMPTY_CONTEXT);
        SqlNode sqlNode = new RelToSqlConverter(dialect).visitChild(0, relNode).asStatement();
        return sqlNode.toSqlString(dialect).getSql();
    }
}
