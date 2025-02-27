import { FormEvent } from "react";

import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

import { useToast } from "@/hooks/use-toast";

import { invoke } from "@tauri-apps/api/core";
import * as log from "@tauri-apps/plugin-log";

export default function Decrypt() {
  const { toast } = useToast();

  async function decryptDatabase(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    log.info("Decrypting database");
    const form = event.target as HTMLFormElement;
    const formData = new FormData(form);
    const password = formData.get("password")!.toString();
    log.debug(`Decrypting with: ${password}`);

    try {
      const status: boolean = await invoke("decrypt_db", { password });

      if (status) {
        toast({
          title: "Success",
          description: "Successfully decrypted database.",
          variant: "default",
        });
        log.info("Successfully decrypted database");
        window.location.reload();
      } else {
        toast({
          title: "Incorrect Password",
          description: "Incorrect password given.",
          variant: "destructive",
        });
        log.info("Incorrect password given");
      }
    } catch (e) {
      toast({
        title: "Error",
        description: `Error when decrypting database: ${e}`,
        variant: "destructive",
      });
      log.error(`Error when decrypting database: ${e}`);
      throw e;
    }
  }

  return (
    <main className="flex h-screen flex-col items-center justify-center">
      <Card className="h-auto">
        <CardHeader>
          <CardTitle>{"Decrypt/Set Password"}</CardTitle>
          <CardDescription>
            {
              "Type in password to the database. This would set the password to the database instead if there is no existing database."
            }
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form id="decrypt" onSubmit={decryptDatabase}>
            <Label htmlFor="password">Password</Label>
            <Input
              required
              id="password"
              name="password"
              type="password"
            ></Input>
          </form>
        </CardContent>
        <CardFooter>
          <Button className="w-full" type="submit" form="decrypt">
            Decrypt
          </Button>
        </CardFooter>
      </Card>
    </main>
  );
}
