import React, { useState } from "react";

import SearchBar from "./SearchBar";
import Medias from "./Medias";

const App = () => {
  const [search, setSearch] = useState("");

  return (
    <div>
      <nav className="navbar navbar-expand-lg navbar-light bg-primary">
        <div className="container-fluid">
          <div className="navbar-brand" href="#">
            <img
              style={{ height: "2rem" }}
              src="/images/logo.svg"
              alt="marshians alien"
            />
          </div>
          <button
            className="navbar-toggler"
            type="button"
            data-bs-toggle="collapse"
            data-bs-target="#navbarSupportedContent"
            aria-controls="navbarSupportedContent"
            aria-expanded="false"
            aria-label="Toggle navigation"
          >
            <span className="navbar-toggler-icon"></span>
          </button>
          <div className="collapse navbar-collapse" id="navbarSupportedContent">
            <ul className="navbar-nav me-auto mb-2 mb-lg-0">
              <li className="nav-item">
                <div className="nav-link active">Marshians Images</div>
              </li>
            </ul>
            <div className="d-flex">
              <SearchBar newSearch={setSearch} />
            </div>
          </div>
        </div>
      </nav>
      <Medias search={search} />
    </div>
  );
};

export default App;
