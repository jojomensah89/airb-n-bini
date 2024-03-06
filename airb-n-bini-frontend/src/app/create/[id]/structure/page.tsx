// import { createCategoryPage } from "@/app/actions";
import { CreatioBottomBar } from "@/components/CreationBottomBar";
import { SelectCategory } from "@/components/SelectCategory";

export default function StrucutreRoute({ params }: { params: { id: string } }) {
  return (
    <>
      <div className="w-3/5 mx-auto">
        <h2 className="text-3xl font-semibold tracking-tight transition-colors">
          Which of these best describe your Home?
        </h2>
      </div>

      <form >
        <input type="hidden" name="homeId" value={params.id} />
        <SelectCategory />

        <CreatioBottomBar />
      </form>
    </>
  );
}
