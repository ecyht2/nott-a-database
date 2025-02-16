import { Suspense, use, useState } from "react";
import { useSearchParams } from "react-router";

import {
  ColumnDef,
  ColumnFiltersState,
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  SortingState,
  useReactTable,
  VisibilityState,
} from "@tanstack/react-table";

import * as log from "@tauri-apps/plugin-log";
import { invoke } from "@tauri-apps/api/core";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";

import { StudentInfo as Student } from "./students";

export type Mark = {
  id: number;
  mark: number;
  fill?: number;
  retake1?: number;
  retake2?: number;
  extra?: string;
  module: string;
  status: string;
};

export type StudentResult = {
  academicYear: string;
  id: number;
  yearOfStudy: number;
  autumnCredits?: number;
  autumnMean?: number;
  springCredits?: number;
  springMean?: number;
  yearCredits?: number;
  yearMean?: number;
  progression?: number;
  remarks?: string;
};

export async function fetchStudent(id: number): Promise<Student> {
  log.info(`Fetching info for ${id}`);
  try {
    const student = (await invoke("get_student", { id })) as Student;
    log.debug(`Marks for ${id}: ${JSON.stringify(student)}`);
    log.info("Done fetching info");
    return student;
  } catch (e) {
    log.error(`Error fetching info for ${id}: ${e}`);
    throw e;
  }
}

export async function fetchMarks(id: number): Promise<Mark[]> {
  log.info(`Fetching marks for ${id}`);
  try {
    const marks = (await invoke("get_marks", { id })) as Mark[];
    log.debug(`Marks for ${id}: ${JSON.stringify(marks)}`);
    log.info("Done fetching marks");
    return marks;
  } catch (e) {
    log.error(`Error fetching marks for ${id}: ${e}`);
    throw e;
  }
}

export async function fetchResults(id: number): Promise<StudentResult[]> {
  log.info(`Fetching results for ${id}`);
  try {
    const results = (await invoke("get_results", { id })) as StudentResult[];
    log.debug(`Results for ${id}: ${JSON.stringify(results)}`);
    log.info("Done fetching results");
    return results;
  } catch (e) {
    log.error(`Error fetching results for ${id}: ${e}`);
    throw e;
  }
}

function GeneralInfo({ info }: { info: Promise<Student> }) {
  const infoData = use(info);
  return (
    <Table className="border-b">
      <TableBody>
        <TableRow>
          <TableHead>ID</TableHead>
          <TableCell>{infoData.id}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>First Name</TableHead>
          <TableCell>{infoData.firstName}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Last Name</TableHead>
          <TableCell>{infoData.lastName}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Career Number</TableHead>
          <TableCell>{infoData.careerNo}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Program</TableHead>
          <TableCell>{infoData.program}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Program Description</TableHead>
          <TableCell>{infoData.programDesc}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Plan Description</TableHead>
          <TableCell>{infoData.planDesc}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Intake</TableHead>
          <TableCell>{infoData.intake}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>QAA</TableHead>
          <TableCell>{infoData.qaa}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Calculation Model</TableHead>
          <TableCell>{infoData.calcModel}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Raw Mark</TableHead>
          <TableCell>{infoData.rawMark}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Truncated Mark</TableHead>
          <TableCell>{infoData.truncatedMark}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Final Mark</TableHead>
          <TableCell>{infoData.finalMark}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Borderline</TableHead>
          <TableCell>{infoData.borderline}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Calculation</TableHead>
          <TableCell>{infoData.calculation}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Degree Award</TableHead>
          <TableCell>{infoData.degreeAward}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Selected</TableHead>
          <TableCell>{infoData.selected}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Exception Data</TableHead>
          <TableCell>{infoData.exceptionData}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Recommendation</TableHead>
          <TableCell>{infoData.recommendation}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Intake Year</TableHead>
          <TableCell>{infoData.intakeYear}</TableCell>
        </TableRow>
        <TableRow>
          <TableHead>Graduation Year</TableHead>
          <TableCell>{infoData.graduationYear}</TableCell>
        </TableRow>
      </TableBody>
    </Table>
  );
}

export const marksColumns: ColumnDef<Mark>[] = [
  {
    accessorKey: "module",
    header: "Module Code",
    cell: ({ row }) => (
      <div className="capitalize">{row.getValue("module")}</div>
    ),
  },
  {
    accessorKey: "mark",
    header: "Initial Mark",
    cell: ({ row }) => <div className="capitalize">{row.getValue("mark")}</div>,
  },
  {
    accessorKey: "status",
    header: "Module Status",
    cell: ({ row }) => (
      <div className="capitalize">{row.getValue("status")}</div>
    ),
  },
  {
    accessorKey: "retake1",
    header: "First Retake",
    cell: ({ row }) => (
      <div className="capitalize">{row.getValue("retake1")}</div>
    ),
  },
  {
    accessorKey: "retake2",
    header: "Second Retake",
    cell: ({ row }) => (
      <div className="capitalize">{row.getValue("retake2")}</div>
    ),
  },
];

function Marks({ marks }: { marks: Promise<Mark[]> }) {
  const data = use(marks);
  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});
  const [rowSelection, setRowSelection] = useState({});

  const table = useReactTable({
    data,
    columns: marksColumns,
    onSortingChange: setSorting,
    onColumnFiltersChange: setColumnFilters,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    onColumnVisibilityChange: setColumnVisibility,
    onRowSelectionChange: setRowSelection,
    state: {
      sorting,
      columnFilters,
      columnVisibility,
      rowSelection,
    },
  });

  return (
    <>
      <Table>
        <TableHeader>
          {table.getHeaderGroups().map((headerGroup) => (
            <TableRow key={headerGroup.id}>
              {headerGroup.headers.map((header) => {
                return (
                  <TableHead key={header.id}>
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext(),
                        )}
                  </TableHead>
                );
              })}
            </TableRow>
          ))}
        </TableHeader>
        <TableBody>
          {table.getRowModel().rows?.length ? (
            table.getRowModel().rows.map((row) => (
              <TableRow
                className="cursor-pointer"
                key={row.id}
                data-state={row.getIsSelected() && "selected"}
              >
                {row.getVisibleCells().map((cell) => (
                  <TableCell key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </TableCell>
                ))}
              </TableRow>
            ))
          ) : (
            <TableRow>
              <TableCell
                colSpan={marksColumns.length}
                className="h-24 text-center"
              >
                No results.
              </TableCell>
            </TableRow>
          )}
        </TableBody>
      </Table>
      <div className="flex items-center justify-end space-x-2 py-4">
        <div className="flex-1 text-sm text-muted-foreground">
          Page {table.getState().pagination.pageIndex + 1} of{" "}
          {table.getPageCount()} page(s).
        </div>
        <div className="space-x-2">
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.previousPage()}
            disabled={!table.getCanPreviousPage()}
          >
            Previous
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.nextPage()}
            disabled={!table.getCanNextPage()}
          >
            Next
          </Button>
        </div>
      </div>
    </>
  );
}

export const resultsColumns: ColumnDef<StudentResult>[] = [
  {
    accessorKey: "yearOfStudy",
    header: "Year",
    cell: ({ row }) => (
      <div className="capitalize">{row.getValue("yearOfStudy")}</div>
    ),
  },
  {
    accessorKey: "academicYear",
    header: "Academic Year",
    cell: ({ row }) => (
      <div className="capitalize">{row.getValue("academicYear")}</div>
    ),
  },
  {
    accessorKey: "progression",
    header: "Progression",
    cell: ({ row }) => (
      <div className="capitalize">{row.getValue("progression")}</div>
    ),
  },
  {
    accessorKey: "autumnMean",
    header: "Mean Marks (Autumn)",
    cell: ({ row }) => (
      <div className="capitalize">{row.getValue("autumnMean")}</div>
    ),
  },
  {
    accessorKey: "springMean",
    header: "Mean Marks (Spring)",
    cell: ({ row }) => (
      <div className="capitalize">{row.getValue("springMean")}</div>
    ),
  },
  {
    accessorKey: "yearMean",
    header: "Mean Marks (Year)",
    cell: ({ row }) => (
      <div className="capitalize">{row.getValue("yearMean")}</div>
    ),
  },
  {
    accessorKey: "autumnCredits",
    header: "Credits Marks (Autumn)",
    cell: ({ row }) => (
      <div className="capitalize">{row.getValue("autumnCredits")}</div>
    ),
  },
  {
    accessorKey: "springCredits",
    header: "Credits Marks (Spring)",
    cell: ({ row }) => (
      <div className="capitalize">{row.getValue("springCredits")}</div>
    ),
  },
  {
    accessorKey: "yearCredits",
    header: "Credits Marks (Year)",
    cell: ({ row }) => (
      <div className="capitalize">{row.getValue("yearCredits")}</div>
    ),
  },
  {
    accessorKey: "remarks",
    header: "Remarks",
    cell: ({ row }) => (
      <div className="capitalize">{row.getValue("remarks")}</div>
    ),
  },
];

function Results({ results }: { results: Promise<StudentResult[]> }) {
  const data = use(results);
  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});
  const [rowSelection, setRowSelection] = useState({});

  const table = useReactTable({
    data,
    columns: resultsColumns,
    onSortingChange: setSorting,
    onColumnFiltersChange: setColumnFilters,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    onColumnVisibilityChange: setColumnVisibility,
    onRowSelectionChange: setRowSelection,
    state: {
      sorting,
      columnFilters,
      columnVisibility,
      rowSelection,
    },
  });

  return (
    <>
      <Table>
        <TableHeader>
          {table.getHeaderGroups().map((headerGroup) => (
            <TableRow key={headerGroup.id}>
              {headerGroup.headers.map((header) => {
                return (
                  <TableHead key={header.id}>
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext(),
                        )}
                  </TableHead>
                );
              })}
            </TableRow>
          ))}
        </TableHeader>
        <TableBody>
          {table.getRowModel().rows?.length ? (
            table.getRowModel().rows.map((row) => (
              <TableRow
                className="cursor-pointer"
                key={row.id}
                data-state={row.getIsSelected() && "selected"}
              >
                {row.getVisibleCells().map((cell) => (
                  <TableCell key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </TableCell>
                ))}
              </TableRow>
            ))
          ) : (
            <TableRow>
              <TableCell
                colSpan={marksColumns.length}
                className="h-24 text-center"
              >
                No results.
              </TableCell>
            </TableRow>
          )}
        </TableBody>
      </Table>
      <div className="flex items-center justify-end space-x-2 py-4">
        <div className="flex-1 text-sm text-muted-foreground">
          Page {table.getState().pagination.pageIndex + 1} of{" "}
          {table.getPageCount()} page(s).
        </div>
        <div className="space-x-2">
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.previousPage()}
            disabled={!table.getCanPreviousPage()}
          >
            Previous
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.nextPage()}
            disabled={!table.getCanNextPage()}
          >
            Next
          </Button>
        </div>
      </div>
    </>
  );
}

export default function StudentInfo() {
  const [searchParams] = useSearchParams();
  const id = parseInt(searchParams.get("id") ?? "");

  if (isNaN(id)) {
    return (
      <article>
        <h1>No ID Provided</h1>
        <p>
          Please provide a vlid student ID for the student in the "id" URL
          search params
        </p>
      </article>
    );
  }

  return (
    <article className="space-y-3">
      <section>
        <Card>
          <CardHeader>
            <CardTitle>General Info</CardTitle>
          </CardHeader>
          <CardContent>
            <Suspense fallback={<div>Loading...</div>}>
              <GeneralInfo info={fetchStudent(id)} />
            </Suspense>
          </CardContent>
        </Card>
      </section>
      <section>
        <Card>
          <CardHeader>
            <CardTitle>Results</CardTitle>
          </CardHeader>
          <CardContent>
            <Suspense fallback={<div>Loading...</div>}>
              <Results results={fetchResults(id)} />
            </Suspense>
          </CardContent>
        </Card>
      </section>
      <section>
        <Card>
          <CardHeader>
            <CardTitle>Marks</CardTitle>
          </CardHeader>
          <CardContent>
            <Suspense fallback={<div>Loading...</div>}>
              <Marks marks={fetchMarks(id)} />
            </Suspense>
          </CardContent>
        </Card>
      </section>
    </article>
  );
}
