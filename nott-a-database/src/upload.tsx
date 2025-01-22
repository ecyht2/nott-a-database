"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { useToast } from "@/hooks/use-toast"

export default function UploadPage() {
    const [file, setFile] = useState<File | null>(null)
    const { toast } = useToast()

    const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (e.target.files) {
            setFile(e.target.files[0])
        }
    }

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault()
        if (!file) {
            toast({
                title: "Error",
                description: "Please select a file to upload.",
                variant: "destructive",
            })
            return
        }

        const formData = new FormData()
        formData.append("file", file)

        try {
            const response = await fetch("/api/upload", {
                method: "POST",
                body: formData,
            })

            if (response.ok) {
                toast({
                    title: "Success",
                    description: "File uploaded successfully!",
                })
                setFile(null)
            } else {
                throw new Error("File upload failed")
            }
        } catch (error) {
            toast({
                title: "Error",
                description: "Failed to upload file. Please try again.",
                variant: "destructive",
            })
        }
    }

    return (
        <Card className="w-full max-w-md mx-auto">
            <CardHeader>
                <CardTitle>Upload Exam Results</CardTitle>
                <CardDescription>Upload a CSV file containing exam results</CardDescription>
            </CardHeader>
            <form onSubmit={handleSubmit}>
                <CardContent>
                    <div className="grid w-full items-center gap-4">
                        <div className="flex flex-col space-y-1.5">
                            <Label htmlFor="file">CSV File</Label>
                            <Input id="file" type="file" accept=".csv" onChange={handleFileChange} />
                        </div>
                    </div>
                </CardContent>
                <CardFooter className="flex justify-between">
                    <Button variant="outline">Cancel</Button>
                    <Button type="submit">Upload</Button>
                </CardFooter>
            </form>
        </Card>
    )
}
