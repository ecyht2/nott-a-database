import "./index.css";

import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Link, Route, Routes } from "react-router";

import Home from "./home";
import UploadPage from "./upload";
import StudentsPage from "./students";
import ModulesPage from "./modules";
import { Toaster } from "@/components/ui/toaster";

ReactDOM.createRoot(document.querySelector("#root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <nav className="p-4 border-b">
        <div className="container mx-auto flex justify-between items-center">
          <h1 className="text-xl font-bold">
            <Link to="/">Nott A Database</Link>
          </h1>
          <ul className="flex gap-x-4">
            <li>
              <Link to="/" className="hover:text-gray-300">
                Home
              </Link>
            </li>
            <li>
              <Link to="/upload" className="hover:text-gray-300">
                Upload
              </Link>
            </li>
            <li>
              <Link to="/students" className="hover:text-gray-300">
                Students
              </Link>
            </li>
            <li>
              <Link to="/modules" className="hover:text-gray-300">
                Modules
              </Link>
            </li>
          </ul>
        </div>
      </nav>
      <main className="m-1">
        <Routes>
          <Route index element={<Home />} />
          <Route path="upload" element={<UploadPage />} />
          <Route path="students" element={<StudentsPage />} />
          <Route path="modules" element={<ModulesPage />} />
        </Routes>
      </main>
    </BrowserRouter>
    <Toaster />
  </React.StrictMode>,
);
