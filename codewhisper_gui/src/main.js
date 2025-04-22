const { readTextFile, BaseDirectory } = window.__TAURI__.fs;

async function loadResults() {
  try {
    const data = await readTextFile('assets/output/result_with_explanations.json', {
      dir: BaseDirectory.Resource,
    });
    

    const results = JSON.parse(data);
    const table = document.createElement("table");
    const thead = document.createElement("thead");
    const headerRow = `
      <tr>
        <th>File</th>
        <th>Line</th>
        <th>Message</th>
        <th>Explanation</th>
      </tr>
    `;
    thead.innerHTML = headerRow;
    table.appendChild(thead);

    const tbody = document.createElement("tbody");

    results.forEach((res) => {
      const row = document.createElement("tr");
      row.innerHTML = `
        <td>${res.file}</td>
        <td>${res.line}</td>
        <td>${res.message}</td>
        <td>${res.explanation}</td>
      `;
      tbody.appendChild(row);
    });

    table.appendChild(tbody);
    document.getElementById("resultTable").innerHTML = "";
    document.getElementById("resultTable").appendChild(table);

  } catch (err) {
    document.getElementById("resultTable").innerText = "⚠️ Could not load results.";
    alert("Load failed: " + err);  // <--- ADD THIS
    console.error(err);
  }

loadResults();
}
