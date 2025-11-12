import { useCallback, useState } from "react";
import { Upload, File, X } from "lucide-react";

interface FileDropzoneProps {
  onFilesSelected: (files: File[]) => void;
  accept?: string;
  multiple?: boolean;
  maxSize?: number; // in MB
}

interface FileWithPreview extends File {
  id: string;
  preview?: string;
}

export function FileDropzone({
  onFilesSelected,
  accept,
  multiple = true,
  maxSize = 100,
}: FileDropzoneProps) {
  const [isDragging, setIsDragging] = useState(false);
  const [selectedFiles, setSelectedFiles] = useState<FileWithPreview[]>([]);

  const handleDragEnter = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
  }, []);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
  }, []);

  const validateFile = (file: File): string | null => {
    const sizeMB = file.size / (1024 * 1024);
    if (sizeMB > maxSize) {
      return `文件大小超过 ${maxSize}MB 限制`;
    }
    return null;
  };

  const processFiles = useCallback((files: File[]) => {
    const validFiles: FileWithPreview[] = [];
    const errors: string[] = [];

    files.forEach((file) => {
      const error = validateFile(file);
      if (error) {
        errors.push(`${file.name}: ${error}`);
      } else {
        validFiles.push({
          ...file,
          id: `${file.name}-${Date.now()}-${Math.random()}`,
        });
      }
    });

    if (errors.length > 0) {
      alert(errors.join("\n"));
    }

    if (validFiles.length > 0) {
      setSelectedFiles((prev) => (multiple ? [...prev, ...validFiles] : validFiles));
      onFilesSelected(validFiles);
    }
  }, [multiple, maxSize, onFilesSelected]);

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);

    const files = Array.from(e.dataTransfer.files);
    processFiles(files);
  }, [processFiles]);

  const handleFileSelect = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files ? Array.from(e.target.files) : [];
    processFiles(files);
  }, [processFiles]);

  const handleRemoveFile = useCallback((fileId: string) => {
    setSelectedFiles((prev) => prev.filter((f) => f.id !== fileId));
  }, []);

  const handleClearAll = useCallback(() => {
    setSelectedFiles([]);
  }, []);

  const formatFileSize = (bytes: number): string => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  };

  return (
    <div className="space-y-4">
      <div
        className={`
          relative border-2 border-dashed rounded-lg p-12
          transition-all duration-200 ease-in-out
          ${isDragging
            ? "border-primary bg-primary/5 scale-[1.02]"
            : "border-border hover:border-primary/50 hover:bg-accent/5"
          }
        `}
        onDragEnter={handleDragEnter}
        onDragLeave={handleDragLeave}
        onDragOver={handleDragOver}
        onDrop={handleDrop}
      >
        <input
          type="file"
          id="file-upload"
          className="hidden"
          accept={accept}
          multiple={multiple}
          onChange={handleFileSelect}
        />

        <label
          htmlFor="file-upload"
          className="flex flex-col items-center justify-center cursor-pointer"
        >
          <Upload
            className={`h-12 w-12 mb-4 transition-colors ${
              isDragging ? "text-primary" : "text-muted-foreground"
            }`}
          />
          <p className="text-lg font-medium mb-2">
            {isDragging ? "放开以上传文件" : "拖拽文件到此处"}
          </p>
          <p className="text-sm text-muted-foreground mb-4">
            或点击选择文件
          </p>
          <p className="text-xs text-muted-foreground">
            {multiple ? "支持多文件上传" : "单文件上传"} · 最大 {maxSize}MB
          </p>
        </label>
      </div>

      {selectedFiles.length > 0 && (
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <h3 className="text-sm font-medium">
              已选择 {selectedFiles.length} 个文件
            </h3>
            <button
              onClick={handleClearAll}
              className="text-xs text-muted-foreground hover:text-foreground"
            >
              清空全部
            </button>
          </div>

          <div className="space-y-2 max-h-64 overflow-y-auto">
            {selectedFiles.map((file) => (
              <div
                key={file.id}
                className="flex items-center gap-3 p-3 rounded-lg border border-border bg-card hover:bg-accent/5 transition-colors"
              >
                <File className="h-5 w-5 text-muted-foreground flex-shrink-0" />
                <div className="flex-1 min-w-0">
                  <p className="text-sm font-medium truncate">{file.name}</p>
                  <p className="text-xs text-muted-foreground">
                    {formatFileSize(file.size)} · {file.type || "未知类型"}
                  </p>
                </div>
                <button
                  onClick={() => handleRemoveFile(file.id)}
                  className="p-1 rounded hover:bg-destructive/10 text-muted-foreground hover:text-destructive transition-colors flex-shrink-0"
                >
                  <X className="h-4 w-4" />
                </button>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
