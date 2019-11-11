package metadata;

import org.apache.calcite.sql.*;
import org.apache.calcite.sql.util.SqlVisitor;

import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class TableNameReader implements SqlVisitor<Set<String>> {

    private Set<String> tableNames = new HashSet<>();

    private void visitBasicCall(SqlBasicCall sqlCall) {
        if (sqlCall.getOperator() instanceof SqlAsOperator && (sqlCall).operands.length == 2) {
            if ((sqlCall).operands[0] instanceof SqlIdentifier
                    && (sqlCall).operands[1] instanceof SqlIdentifier) {
                (sqlCall).operands[0].accept(this);
            } else if (!((sqlCall).operands[0] instanceof SqlIdentifier)) {
                (sqlCall).operands[0].accept(this);
            }
        } else {
            Arrays.stream((sqlCall).operands).forEach((node) -> {
                if (node instanceof SqlSelect) {
                    if (((SqlSelect) node).getFrom() != null) {
                        ((SqlSelect) node).getFrom().accept(this);
                    }
                }

                if (node instanceof SqlBasicCall) {
                    visitBasicCall((SqlBasicCall) node);
                }
            });
        }
    }

    @Override
    public Set<String> visit(SqlLiteral literal) {
        return tableNames;
    }

    @Override
    public Set<String> visit(SqlCall sqlCall) {
        if (sqlCall instanceof SqlSelect) {
            ((SqlSelect) sqlCall).getSelectList().accept(this);
            if (((SqlSelect) sqlCall).getFrom() != null) {
                ((SqlSelect) sqlCall).getFrom().accept(this);
            }
            if (((SqlSelect) sqlCall).getWhere() instanceof SqlBasicCall) {
                List<SqlNode> operands =
                        ((SqlBasicCall) ((SqlSelect) sqlCall).getWhere()).getOperandList();
                for (SqlNode operand : operands) {
                    if (!(operand instanceof SqlIdentifier)) {
                        operand.accept(this);
                    }
                }
            }
        }

        if (sqlCall instanceof SqlJoin) {
            ((SqlJoin) sqlCall).getLeft().accept(this);
            ((SqlJoin) sqlCall).getRight().accept(this);
        }

        if (sqlCall instanceof SqlBasicCall) {
            visitBasicCall((SqlBasicCall) sqlCall);
        }

        if (sqlCall instanceof SqlOrderBy) {
            ((SqlOrderBy) sqlCall).query.accept(this);
        }

        return tableNames;
    }

    @Override
    public Set<String> visit(SqlNodeList nodeList) {
        nodeList.iterator().forEachRemaining((entry) -> {
            if (entry instanceof SqlSelect) {
                entry.accept(this);
            } else if (entry instanceof SqlBasicCall) {
                String kind = ((SqlBasicCall) entry).getOperator().getName();
                if ("AS".equalsIgnoreCase(kind)
                        && ((SqlBasicCall) entry).operand(0) instanceof SqlSelect) {
                    entry.accept(this);
                }
            }
        });
        return tableNames;
    }

    @Override
    public Set<String> visit(SqlIdentifier id) {
        if (id.names.size() == 0) {
            return tableNames;
        }

        tableNames.add(id.toString().toLowerCase());
        return tableNames;
    }

    @Override
    public Set<String> visit(SqlDataTypeSpec type) {
        return tableNames;
    }

    @Override
    public Set<String> visit(SqlDynamicParam param) {
        return tableNames;
    }

    @Override
    public Set<String> visit(SqlIntervalQualifier intervalQualifier) {
        return tableNames;
    }
}
