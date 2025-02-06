import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { invoke } from "@tauri-apps/api/core";
import { Suspense, useEffect, useState } from "react";
import * as log from "@tauri-apps/plugin-log";

type Module = {
  code: string;
  credit: number;
  name?: string;
};

function ModuleRows() {
  const [modules, setModules] = useState<Module[] | null>(null);
  useEffect(() => {
    async function fetchModules() {
      log.info("Fetching module data");
      try {
        const modules = (await invoke("get_modules")) as Module[];
        log.info("Done fetching module data");
        log.debug(`Module Data: ${JSON.stringify(modules)}`);
        setModules(modules);
      } catch (e) {
        log.error(`Error fetching module data: ${e}`);
      }
    }

    fetchModules();
  }, []);

  return modules?.map((result) => (
    <TableRow key={result.code}>
      <TableCell>{result.code}</TableCell>
      <TableCell>{result.credit}</TableCell>
      <TableCell>{result.name ?? ""}</TableCell>
    </TableRow>
  ));
}

export default function ModulesPage() {
  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Module Code</TableHead>
            <TableHead>Module Credits</TableHead>
            <TableHead>Module Name</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <Suspense fallback={<div>Loading</div>}>
            <ModuleRows />
          </Suspense>
        </TableBody>
      </Table>
    </div>
  );
}
