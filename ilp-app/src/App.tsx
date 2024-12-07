import { useEffect, useState } from "react";
import "./App.css";
import { invoke } from "@tauri-apps/api";

const messages = {
  "Contigent": "A formula é contigente",
  "Contradiction": "A formula é uma contradição",
  "Tautology": "A formula é uma tautologia"
}

function App() {
  const [expression, setExpression] = useState("");
  const [expressionVisible, setExpressionVisible] = useState("");

  const [message, setMessage] = useState<string>();

  const [tableHeader, setTableHeader] = useState<string[]>([]);
  const [table, setTable] = useState<Record<string, boolean>[]>([]);

  useEffect(() => {
    if (table.length > 0) {
      invoke("get_type", { input: expression.trim(), table }).then((type) =>
        setMessage(messages[type as keyof typeof message])
      )

      const _header = Object.keys(table[0]);
      _header.sort();
      
      setTableHeader(_header);
      setExpressionVisible(expression);

    }
  }, [table]);

  async function generateTable() {
    setTable([])
    setMessage("")
    try {
      const _table = (await invoke("get_table", { input: expression.trim() })) as Record<
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
    } catch (err) { 
      setTableHeader([])
      setTable([])
      setMessage(err as string)
     }
  }

  return (
    <div className="container">
      <div className="row">
        <input
          onChange={(e) => setExpression(e.target.value)}
          style={{ marginRight: 8 }}
        />

        <button onClick={generateTable}>OK</button>
      </div>
      <p>{message}</p>
      <table className="table">
        <thead>
        <tr>
          {tableHeader.map((header) => (
            <th id={header}>{header == "result" ? expressionVisible : header}</th>
          ))}
        </tr>
        </thead>
        <tbody>
        {table.map((row, index) => (
          <tr key={index}>
            {tableHeader.map((header, index) => (
              <td key={index}>{row[header] ? "V" : "F"}</td>
            ))}
          </tr>
        ))}
        </tbody>
      </table>
    </div>
  );
}

export default App;
