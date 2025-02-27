import "./index.css";

import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Link, Route, Routes } from "react-router";

import Decrypt from "./decrypt";
import Home from "./home";
import UploadPage from "./upload";
import StudentsPage from "./students";
import ModulesPage from "./modules";
import StudentInfo from "./student_info";
import Settings from "./settings";
import { Toaster } from "@/components/ui/toaster";
import { ThemeProvider } from "@/components/theme-provider";
import { ModeToggle } from "@/components/mode-toggle";

import { invoke } from "@tauri-apps/api/core";

function Main() {
  const [decrypted, setDecrypted] = useState<boolean>(false);
  useEffect(function () {
    async function checkDecryption() {
      setDecrypted(await invoke("check_decryption"));
    }
    checkDecryption();
  });

  if (!decrypted) {
    return <Decrypt />;
  }

  return (
    <>
      <nav className="border-b p-4">
        <div className="container mx-auto flex items-center gap-x-4">
          <h1 className="text-xl font-bold">
            <Link to="/">Nott A Database</Link>
          </h1>
          <ul className="ml-auto flex gap-x-4">
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
            <li>
              <Link to="/settings" className="hover:text-gray-300">
                Settings
              </Link>
            </li>
          </ul>
          <ModeToggle />
        </div>
      </nav>
      <main className="m-1">
        <Routes>
          <Route index element={<Home />} />
          <Route path="upload" element={<UploadPage />} />
          <Route path="students" element={<StudentsPage />} />
          <Route path="student" element={<StudentInfo />} />
          <Route path="modules" element={<ModulesPage />} />
          <Route path="settings" element={<Settings />} />
        </Routes>
      </main>
    </>
  );
}

ReactDOM.createRoot(document.querySelector("#root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider>
      <BrowserRouter>
        <Main />
      </BrowserRouter>
      <Toaster />
    </ThemeProvider>
  </React.StrictMode>,
);
