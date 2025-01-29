import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

// This is a mock function. In a real application, you would fetch this data from your backend.
async function getExamResults() {
  return [
    { id: 1, name: "John Doe", subject: "Math", score: 85 },
    { id: 2, name: "Jane Smith", subject: "Science", score: 92 },
    { id: 3, name: "Bob Johnson", subject: "History", score: 78 },
  ];
}
const results = await getExamResults();

export default function ModulesPage() {
  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Name</TableHead>
            <TableHead>Subject</TableHead>
            <TableHead>Score</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {results.map((result) => (
            <TableRow key={result.id}>
              <TableCell>{result.name}</TableCell>
              <TableCell>{result.subject}</TableCell>
              <TableCell>{result.score}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}
