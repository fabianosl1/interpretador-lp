import { useEffect, useState } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api";

function App() {
  const [expression, setExpression] = useState("");
  const [expressionType, setExpressionType] = useState<string>();

  const [tableHeader, setTableHeader] = useState<string[]>([]);
  const [table, setTable] = useState<Record<string, boolean>[]>([]);

  useEffect(() => {
    if (table.length > 0) {
      invoke("get_type", { input: expression, table }).then((type) =>
        setExpressionType(type as string)
      );

      const _header = Object.keys(table[0]);
      _header.sort();

      setTableHeader(_header);
    }
  }, [table]);

  async function generateTable() {
    const _table = (await invoke("get_table", { input: expression })) as Record<
      string,
      boolean
    >[];

    _table.sort((a, b) => {
      const keys = Object.keys(a).filter(key => key !== "result").sort();

      for (let i = 0; i < keys.length; i++) {
        const key = keys[i]

        if (a[key] !== b[key]) {
          return a[key] ? -1 : 1
        }
      }

      return 0;
    });
    
    setTable(_table);
  }

  return (
    <div className="container">
      <div className="row">
        <input
          onChange={(e) => setExpression(e.target.value)}
          style={{ marginRight: 8 }}
        />

        <button onClick={generateTable}>Gerar tabela</button>
      </div>
      <p>{expressionType}</p>
      <table>
        <tr>
          {tableHeader.map((header) => (
            <th id={header}>{header == "result" ? expression : header}</th>
          ))}
        </tr>
        {table.map((row, index) => (
          <tr key={index}>
            {tableHeader.map((header, index) => (
              <td key={index}>{row[header] ? "V" : "F"}</td>
            ))}
          </tr>
        ))}
      </table>
    </div>
  );
}

export default App;
