package sql.handler;

import org.apache.calcite.rel.RelNode;
import org.apache.calcite.schema.SchemaPlus;

import java.io.IOException;
import java.util.Set;

public interface SqlHandler {

    SchemaPlus createSchema(SchemaPlus rootSchema, Set<String> tableNames) throws IOException;

    String createSql(RelNode relNode);

}
