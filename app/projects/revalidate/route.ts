import { revalidatePath } from "next/cache";

export async function GET() {
  return revalidatePath("/projects");
}
