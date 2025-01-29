"use client";

import { ChangeEvent, useState } from "react";
import { Button } from "@/components/ui/button";
import { Input, FileInput } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectLabel,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import { useToast } from "@/hooks/use-toast";
import * as log from "@tauri-apps/plugin-log";
import { invoke } from "@tauri-apps/api/core";

export default function UploadPage() {
    const [type, setType] = useState<string | null>(null);
    const [year, setYear] = useState<string | null>(null);
    const [file, setFile] = useState<string | null>(null);
    const { toast } = useToast();

    function handleTypeChange(type: string) {
        setType(type);
        if (type !== null) {
            log.info(`Type: ${type} selected for data`);
        }
    }

    function handleYearChange(event: ChangeEvent) {
        if (event.target === null) {
            return;
        }
        const element: HTMLInputElement = event.target as any;
        let year: string | null = element.value;

        if (year.length === 0) {
            year = null;
        }

        setYear(year);
        if (year !== null) {
            log.info(`Academic Year of the selected data is ${year}`);
        }
    }

    function handleFileChange(fileName: string | null) {
        setFile(fileName);

        if (fileName !== null) {
            log.info(`File: ${fileName} selected for upload`);
        }
    }

    async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault();

        if (!type) {
            log.error("Tried to upload without data type");
            toast({
                title: "Error",
                description: "Please select the type of data.",
                variant: "destructive",
            });
            return;
        }

        if (!year) {
            log.error("Tried to upload without academic year");
            toast({
                title: "Error",
                description: "Please state the academic year of the data.",
                variant: "destructive",
            });
            return;
        } else {
            const years = year.split("/");

            function showError(reason: string) {
                const description = `Invalid academic year, ${reason} e.g. 2024/2025.`;
                log.error(description);
                toast({
                    title: "Error",
                    description: description,
                    variant: "destructive",
                });
            }

            if (years.length !== 2) {
                showError("academic year should have two numbers separated by \"/\"");
                return;
            }

            const start = parseInt(years[0]);
            const end = parseInt(years[1]);

            if (isNaN(start)) {
                showError(`invalid start year \"${years[0]}\" found`);
                return;
            } else if (isNaN(end)) {
                showError(`invalid end year \"${years[1]}\" found`);
                return;
            } else {
                if (start + 1 !== end) {
                    showError(`the end year of the academic year should be 1 more than the start year`);
                    return;
                }
            }
        }

        if (!file) {
            log.error("Tried to upload without a file");
            toast({
                title: "Error",
                description: "Please select a file to upload.",
                variant: "destructive",
            });
            return;
        }

        try {
            log.info("Uploading Data...");
            log.debug(`Form Data\nType: ${type}\nAcademic Year: ${year}\nFile: ${file}`);
            await invoke("insert_data", { dataType: type, academicYear: year, path: file });
            log.info("Successfully Uploaded Data");
            toast({
                "title": "Success",
                "description": "Successfully uploaded data",
            });
        } catch (error) {
            log.error(`${error}`);
            toast({
                title: "Error",
                description: `An error has occured: ${error}`,
                variant: "destructive",
            });
        }
    }

    return (
        <Card className="w-full max-w-md mx-auto">
            <CardHeader>
                <CardTitle>Upload Exam Results</CardTitle>
                <CardDescription>
                    Upload a CSV file containing exam results
                </CardDescription>
            </CardHeader>
            <form onSubmit={handleSubmit}>
                <CardContent>
                    <div className="grid w-full items-center gap-4">
                        <div className="flex flex-col space-y-1.5 max-w-full">
                            <Label htmlFor="type">Data Type</Label>
                            <Select onValueChange={handleTypeChange} name="type">
                                <SelectTrigger>
                                    <SelectValue placeholder="Select a Data Type" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectGroup>
                                        <SelectLabel>Data Type</SelectLabel>
                                        <SelectItem value="result">Result (0A)</SelectItem>
                                        <SelectItem value="award">Award (0B)</SelectItem>
                                        <SelectItem value="resit-may">May Resit (0C)</SelectItem>
                                        <SelectItem value="resit-aug">August Resit (0D)</SelectItem>
                                    </SelectGroup>
                                </SelectContent>
                            </Select>
                        </div>
                        <div className="flex flex-col space-y-1.5 max-w-full">
                            <Label htmlFor="year">Academic Year</Label>
                            <Input
                                id="year"
                                name="year"
                                onChange={handleYearChange}
                            />
                        </div>
                        <div className="flex flex-col space-y-1.5 max-w-full">
                            <Label htmlFor="file">XLSX File</Label>
                            <FileInput
                                name="file"
                                id="file"
                                accept=".xlsx"
                                clickFn={handleFileChange}
                            />
                        </div>
                    </div>
                </CardContent>
                <CardFooter>
                    <Button type="submit" className="w-full">Upload</Button>
                </CardFooter>
            </form>
        </Card>
    );
}
