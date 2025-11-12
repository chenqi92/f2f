import { Routes, Route, Navigate } from "react-router-dom";
import { MainLayout } from "@/components/layouts/MainLayout";
import { QuickConvert } from "@/pages/QuickConvert";
import { RecipeBuilder } from "@/pages/RecipeBuilder";
import { Queue } from "@/pages/Queue";
import { Preview } from "@/pages/Preview";
import { Plugins } from "@/pages/Plugins";
import { Settings } from "@/pages/Settings";

export function AppRouter() {
  return (
    <Routes>
      <Route path="/" element={<MainLayout />}>
        <Route index element={<Navigate to="/convert" replace />} />
        <Route path="convert" element={<QuickConvert />} />
        <Route path="recipes" element={<RecipeBuilder />} />
        <Route path="queue" element={<Queue />} />
        <Route path="preview" element={<Preview />} />
        <Route path="plugins" element={<Plugins />} />
        <Route path="settings" element={<Settings />} />
      </Route>
    </Routes>
  );
}
