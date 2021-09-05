import React, { useEffect, useState } from "react";

const Table = (props: { chores: []; rarity: String }) => {
  const [vis, setVis] = useState(false);
  const [chores, setChores] = useState([]);

  useEffect(() => {
    setChores(props.chores);
  }, [props.chores]);

  return (
    <div
      className="commonTable"
      style={{ display: "flex", flexDirection: "column" }}
    >
      <div
        className="header"
        style={{ display: "flex", flex: 1, flexDirection: "row" }}
      >
        <div style={{ fontSize: "30px" }}>{props.rarity}</div>
        <div
          style={{
            display: "flex",
            cursor: "pointer",
            flex: 1,
            fontSize: "50px",
            justifyContent: "flex-end",
            width: "50px",
          }}
          onClick={(_e) => {
            setVis(!vis);
          }}
        >
          {vis ? "v" : "Λ"}
        </div>
      </div>

      {chores.map((text, i) => (
        <div
          style={{
            border: "1px solid rgba(9,5,14,80)",
            minHeight: "35px",
            display: vis ? "flex" : "none",
            flexDirection: "row",
          }}
        >
          <div
            style={{
              borderRight: "1px solid rgba(69,69,69,69)",
              display: "flex",
              flex: 1,
              margin: "10px",
              maxWidth: "35px",
            }}
          >
            {i}
          </div>
          <div
            style={{
              display: "flex",
              flex: 1,
              justifyContent: "flex-start",
              margin: "10px",
            }}
          >
            {text}
          </div>
        </div>
      ))}
    </div>
  );
};

export default Table;
