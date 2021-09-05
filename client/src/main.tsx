import React, { useEffect, useState } from "react";
import axios from "axios";

import Table from "./table";

const Main = () => {
  const [chores, setChores]: [
    { common: []; uncommon: []; rare: []; spicy: [] },
    Function
  ] = useState({
    common: [],
    uncommon: [],
    rare: [],
    spicy: [],
  });

  useEffect(() => {
    axios.get("http://wonk.gg:8080/read_all").then((res) => {
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
        <Table chores={chores.common} rarity={"common"} />
        <Table chores={chores.uncommon} rarity={"uncommon"} />
        <Table chores={chores.rare} rarity={"rare"} />
        <Table chores={chores.spicy} rarity={"spicy"} />
      </div>
    </div>
  );
};

export default Main;
