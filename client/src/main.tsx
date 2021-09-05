import React, { useEffect, useState } from "react";
import axios from "axios";

const Main = () => {
  const [chores, setChores] = useState({
    common: [],
    uncommon: [],
    rare: [],
    spicy: [],
  });
  const [visibiliy, setVisibility] = useState({
    common: false,
    uncommon: false,
    rare: false,
    spicy: false,
  });

  useEffect(() => {
    axios.get("http://localhost:8080/read_all").then((res) => {
      console.log(res.data);
      setChores(res.data);
      return res;
    });
  }, []);

  return (
    <div
      className="main"
      style={{
        backgroundColor: "#31343a",
        color: "#ddd",
        display: "flex",
        fontWeight: "bold",
        fontFamily: "Helvetica",
      }}
    >
      <div
        className="listOfTables"
        style={{
          display: "flex",
          flexDirection: "column",
          padding: "50px",
          width: "600px",
        }}
      >
        <div
          className="commonTable"
          style={{ display: "flex", flexDirection: "column" }}
        >
          <div
            className="header"
            style={{ display: "flex", flex: 1, flexDirection: "row" }}
          >
            <div style={{fontSize: "30px"}}>Common</div>
            <div
              style={{ display: "flex", cursor: 'pointer', flex: 1, fontSize: "50px", justifyContent: "flex-end", width: '50px' }}
              onClick={(e) => {
                console.log("click");
                setVisibility({ ...visibiliy, common: !visibiliy.common });
              }}
            >
              {visibiliy.common ? "v" : "Λ"}
            </div>
          </div>

          {chores.common.map((text, i) => (
            <div
              style={{
                border: "1px solid rgba(9,5,14,80)",
                minHeight: "35px",
                display: visibiliy.common ? "flex" : "none",
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
      </div>
    </div>
  );
};

export default Main;
