import * as React from "react";

import { cn } from "@/lib/utils";
import { DialogFilter, open } from "@tauri-apps/plugin-dialog";
import { Button } from "./button";

const Input = React.forwardRef<HTMLInputElement, React.ComponentProps<"input">>(
  ({ className, type, ...props }, ref) => {
    return (
      <input
        type={type}
        className={cn(
          "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-base shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-inset focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
          className,
        )}
        ref={ref}
        {...props}
      />
    );
  },
);
Input.displayName = "Input";

type FileInputProps = React.ComponentProps<"button"> & {
  accept?: string;
  clickFn: (fileName: string | null) => void;
};

function FileInput({ className, accept, clickFn, ...props }: FileInputProps) {
  const [file, setFile] = React.useState<string | null>(null);

  const filters: DialogFilter[] = [];
  if (accept !== undefined) {
    filters.push({
      name: accept,
      extensions: accept.split(",").map((extension) => {
        if (extension.startsWith(".")) {
          return extension.substring(1);
        }

        return extension;
      }),
    });
  }

  return (
    <Button
      variant="outline"
      className={cn(
        "flex w-full items-start justify-start gap-3 hover:bg-transparent focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-inset focus-visible:ring-ring hover:outline-none hover:ring-1 hover:ring-inset hover:ring-ring disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        className,
      )}
      onClick={async () => {
        const file = await open({
          multiple: false,
          directory: false,
          filters: filters,
        });
        setFile(file);
        clickFn(file);
      }}
      {...props}
    >
      <span>Choose File</span>
      <span className="truncate">{file ?? "no file selected"}</span>
    </Button>
  );
}
FileInput.displayName = "Input";

export { Input, FileInput };
