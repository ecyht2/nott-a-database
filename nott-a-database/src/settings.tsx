import { FormEvent, useEffect, useRef, useState } from "react";

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

import * as log from "@tauri-apps/plugin-log";
import { invoke } from "@tauri-apps/api/core";

function ChangePassword() {
  const [passwd, setPasswd] = useState<string>("");
  const [confirmPasswd, setConfirmPasswd] = useState<string>("");
  const confirmRef = useRef<HTMLInputElement>(null);
  const { toast } = useToast();

  async function handleChangePassword(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const formData = new FormData(event.target as HTMLFormElement);
    const password = formData.get("passwd")!.toString();
    log.info("Changing database password");
    log.debug(`New password: ${password}`);
    try {
      await invoke("change_password", {
        password,
      });
      toast({
        title: "Success",
        description: "Successfully updated password to the database.",
        variant: "default",
      });
      log.info("Done updating password");
    } catch (e) {
      toast({
        title: "Error",
        description: `Error changing password: ${e}`,
        variant: "default",
      });
      log.error(`Error changing password: ${e}`);
      throw e;
    }
  }

  function checkEqual() {
    log.info("Checking confirm password validity.");
    log.debug(`New Password: ${passwd}`);
    log.debug(`Confirmed Password: ${confirmPasswd}`);

    if (confirmPasswd !== passwd) {
      log.info("Password and confirm password does not match.");
      confirmRef.current?.setCustomValidity(
        "Confirm password must match new password",
      );
    } else {
      log.info("Password and confirm password does match!");
      confirmRef.current?.setCustomValidity("");
    }
  }

  useEffect(checkEqual, [passwd, confirmPasswd]);

  return (
    <article>
      <Card>
        <CardHeader>
          <CardTitle>Change Password</CardTitle>
          <CardDescription>
            Change the password to the database.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form id="change-passwd" onSubmit={handleChangePassword}>
            <Label htmlFor="new-passwd">New Password</Label>
            <Input
              required
              value={passwd}
              onChange={(e) => {
                setPasswd(e.target.value);
              }}
              type="password"
              id="new-passwd"
              name="passwd"
            />
            <Label htmlFor="confirm-passwd">Confirm Password</Label>
            <Input
              required
              type="password"
              id="old-passwd"
              ref={confirmRef}
              value={confirmPasswd}
              onChange={(e) => {
                setConfirmPasswd(e.target.value);
              }}
            />
          </form>
        </CardContent>
        <CardFooter>
          <Button type="submit" form="change-passwd" className="w-full">
            Change Password
          </Button>
        </CardFooter>
      </Card>
    </article>
  );
}

export default function Settings() {
  return (
    <article>
      <h1 className="p-2 text-xl font-bold">Settings</h1>
      <p className="p-2 text-sm text-muted-foreground">
        Manage database settings and preferences.
      </p>
      <ChangePassword />
    </article>
  );
}
