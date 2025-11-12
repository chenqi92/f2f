import { NavLink } from "react-router-dom";
import {
  Zap,
  BookOpen,
  ListTodo,
  Eye,
  Puzzle,
  Settings,
} from "lucide-react";

const navItems = [
  { to: "/convert", icon: Zap, label: "快速转换" },
  { to: "/recipes", icon: BookOpen, label: "食谱" },
  { to: "/queue", icon: ListTodo, label: "队列" },
  { to: "/preview", icon: Eye, label: "预览" },
  { to: "/plugins", icon: Puzzle, label: "插件" },
  { to: "/settings", icon: Settings, label: "设置" },
];

export function Sidebar() {
  return (
    <aside className="w-64 border-r border-border bg-card">
      <div className="flex h-16 items-center border-b border-border px-6">
        <h1 className="text-xl font-semibold">F2F Converter</h1>
      </div>
      <nav className="space-y-1 p-4">
        {navItems.map((item) => (
          <NavLink
            key={item.to}
            to={item.to}
            className={({ isActive }) =>
              `flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors ${
                isActive
                  ? "bg-primary text-primary-foreground"
                  : "text-muted-foreground hover:bg-accent hover:text-accent-foreground"
              }`
            }
          >
            <item.icon className="h-5 w-5" />
            {item.label}
          </NavLink>
        ))}
      </nav>
    </aside>
  );
}
