import { Link } from "react-router";

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";

export function Home() {
    return (
        <div className="grid gap-6 md:grid-cols-2">
            <Card>
                <CardHeader>
                    <CardTitle>Upload Exam Results</CardTitle>
                    <CardDescription>Submit new exam results data</CardDescription>
                </CardHeader>
                <CardContent>
                    <Link to="/upload">
                        <Button>Go to Upload</Button>
                    </Link>
                </CardContent>
            </Card>
            <Card>
                <CardHeader>
                    <CardTitle>View Students</CardTitle>
                    <CardDescription>Check students results</CardDescription>
                </CardHeader>
                <CardContent>
                    <Link to="/students">
                        <Button>View Students</Button>
                    </Link>
                </CardContent>
            </Card>
            <Card>
                <CardHeader>
                    <CardTitle>View Modules</CardTitle>
                    <CardDescription>Check/Edit recorded modules in the database</CardDescription>
                </CardHeader>
                <CardContent>
                    <Link to="/modules">
                        <Button>View Modules</Button>
                    </Link>
                </CardContent>
            </Card>
        </div>
    )
}

export default Home;
