package sql.handler;

import com.google.common.collect.ImmutableList;
import exception.ParseException;
import metadata.ResourceReader;
import org.apache.calcite.adapter.druid.DruidQuery;
import org.apache.calcite.adapter.druid.DruidSchemaFactory;
import org.apache.calcite.adapter.enumerable.EnumerableConvention;
import org.apache.calcite.plan.*;
import org.apache.calcite.rel.RelNode;
import org.apache.calcite.rel.RelVisitor;
import org.apache.calcite.rel.core.TableScan;
import org.apache.calcite.schema.Schema;
import org.apache.calcite.schema.SchemaPlus;
import org.apache.calcite.tools.Program;
import org.apache.calcite.tools.Programs;
import sql.enums.SqlResultType;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;

public class DruidHandler implements SqlHandler {
    @Override
    public SchemaPlus createSchema(SchemaPlus rootSchema, Set<String> tableNames) throws IOException {
        Map<String, Object> druidConfig = ResourceReader.getJdbcResources(SqlResultType.DRUID);
        if (!tableNames.isEmpty()) {
            for (String tableName : tableNames) {
                Map<String, Object> configMap = new HashMap<>(druidConfig);
                String[] split = tableName.split("\\.");
                Schema schema = new DruidSchemaFactory().create(rootSchema, split[1], configMap);
                rootSchema.add(split[0], schema);
            }
        } else {
            throw new ParseException("druid table is null!");
        }
        return rootSchema;
    }

    @Override
    public String createSql(RelNode relNode) {
        RelOptPlanner planner = relNode.getCluster().getPlanner();
        final RelVisitor visitor = new RelVisitor() {
            @Override
            public void visit(RelNode node, int ordinal, RelNode parent) {
                if (node instanceof TableScan) {
                    final RelOptCluster cluster = node.getCluster();
                    final RelOptTable.ToRelContext context = ViewExpanders.simpleContext(cluster);
                    RelNode r = node.getTable().toRel(context);
                    planner.registerClass(r);
                }
                super.visit(node, ordinal, parent);
            }
        };
        visitor.go(relNode);
        Program program = Programs.standard();
        RelTraitSet traits = planner.emptyTraitSet().replace(EnumerableConvention.INSTANCE);
        RelNode node = program.run(planner, relNode, traits, ImmutableList.of(), ImmutableList.of());
        DruidQuery query = (DruidQuery) node.getInput(0);
        return query.getQuerySpec().getQueryString(null, 0);
    }
}
