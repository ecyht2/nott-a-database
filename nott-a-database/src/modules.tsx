import { FormEvent, Suspense, useEffect, useState } from "react";

import { Edit } from "lucide-react";

import { invoke } from "@tauri-apps/api/core";
import * as log from "@tauri-apps/plugin-log";

import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogFooter,
  DialogHeader,
  DialogContent,
  DialogDescription,
  DialogTitle,
  DialogTrigger,
  DialogClose,
} from "@/components/ui/dialog";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useToast } from "./hooks/use-toast";

type Module = {
  code: string;
  credit: number;
  name?: string;
};

function EditModule({
  module,
  update,
}: {
  module: Module;
  update: (module: Module) => void;
}) {
  const { toast } = useToast();

  async function updateModule(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const dataForm = new FormData(event.target as HTMLFormElement);
    const data: Module = {
      code: dataForm.get("code")!.toString(),
      credit: parseInt(dataForm.get("credit")!.toString()),
      name: dataForm.get("name")?.toString(),
    };

    log.info(`Updating module ${data.code}`);
    log.debug(`New Credits: ${data.credit}, New Name: ${data.name}`);

    if (isNaN(data.credit) || data.credit <= 0) {
      const description = "Please set module credits that is > 0";
      log.error("Error updating module: " + description);
      toast({
        title: "Error",
        description: description,
        variant: "destructive",
      });
      return;
    }

    try {
      const newModule = (await invoke("update_module", {
        module: data,
      })) as Module;
      log.info(`Successfully updated module ${data.code}`);
      toast({
        title: "Success",
        description: `Successfully updated module ${data.code}`,
      });
      update(newModule);
    } catch (e) {
      toast({
        title: "Error",
        description: `${e}`,
        variant: "destructive",
      });
      log.error(`Error updating module ${data.code}: ${e}`);
    }
  }

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button size="icon" variant="ghost">
          <Edit />
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{"Editing Module"}</DialogTitle>
          <DialogDescription>
            {`Making changes to ${module.code}. Click "Save" when you are done.`}
          </DialogDescription>
        </DialogHeader>
        <form onSubmit={updateModule} id="form" className="flex flex-col gap-4">
          <Input
            value={module.code}
            type="hidden"
            id="code-input"
            name="code"
          />
          <Label htmlFor="credits-input">New Credits</Label>
          <Input
            id="credit-input"
            name="credit"
            type="number"
            defaultValue={module.credit}
          />
          <Label htmlFor="name-input">New Name</Label>
          <Input
            id="name-input"
            name="name"
            defaultValue={module.name}
            placeholder="Some Module Name"
          />
        </form>
        <DialogFooter>
          <DialogClose asChild>
            <Button form="form" type="submit">
              Save
            </Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

function ModuleRows() {
  const [modules, setModules] = useState<Module[]>([]);
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

  if (modules.length === 0) {
    return (
      <TableRow>
        <TableCell colSpan={3} className="text-muted-foreground">
          {"No modules found. Please upload data in the Upload page."}
        </TableCell>
      </TableRow>
    );
  }

  return modules.map((result, idx) => (
    <TableRow key={result.code}>
      <TableCell>{result.code}</TableCell>
      <TableCell>{result.credit}</TableCell>
      <TableCell>{result.name ?? ""}</TableCell>
      <TableCell className="max-w-1">
        <EditModule
          module={result}
          update={(module) => {
            modules[idx] = module;
            const newModules = modules.slice();
            setModules(newModules);
          }}
        />
      </TableCell>
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
            <TableHead className="max-w-1"></TableHead>
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
