package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
	"time"
)

type FileOrganizer struct {
	sourceDir      string
	targetDir      string
	organizeBy     string
	dryRun         bool
	recursive      bool
	movedCount     int
	skippedCount   int
}

var extensionCategories = map[string]string{
	//documents
	".pdf":  "Documents",
	".doc":  "Documents",
	".docx": "Documents",
	".txt":  "Documents",
	".rtf":  "Documents",
	".odt":  "Documents",
	".xlsx": "Documents",
	".xls":  "Documents",
	".pptx": "Documents",
	".ppt":  "Documents",
	
	//images
	".jpg":  "Images",
	".jpeg": "Images",
	".png":  "Images",
	".gif":  "Images",
	".bmp":  "Images",
	".svg":  "Images",
	".webp": "Images",
	".ico":  "Images",
	
	//videos
	".mp4":  "Videos",
	".avi":  "Videos",
	".mkv":  "Videos",
	".mov":  "Videos",
	".wmv":  "Videos",
	".flv":  "Videos",
	".webm": "Videos",
	
	//audio
	".mp3":  "Audio",
	".wav":  "Audio",
	".flac": "Audio",
	".aac":  "Audio",
	".ogg":  "Audio",
	".m4a":  "Audio",
	
	//archives
	".zip":  "Archives",
	".rar":  "Archives",
	".7z":   "Archives",
	".tar":  "Archives",
	".gz":   "Archives",
	
	//code
	".go":   "Code",
	".py":   "Code",
	".js":   "Code",
	".java": "Code",
	".cpp":  "Code",
	".c":    "Code",
	".html": "Code",
	".css":  "Code",
	".json": "Code",
	".xml":  "Code",
}

func NewFileOrganizer(source, target, organizeBy string, dryRun, recursive bool) *FileOrganizer {
	return &FileOrganizer{
		sourceDir:  source,
		targetDir:  target,
		organizeBy: organizeBy,
		dryRun:     dryRun,
		recursive:  recursive,
	}
}

func (fo *FileOrganizer) Organize() error {
	fmt.Printf("Starting file organization...\n")
	fmt.Printf("Source: %s\n", fo.sourceDir)
	fmt.Printf("Target: %s\n", fo.targetDir)
	fmt.Printf("Organize by: %s\n", fo.organizeBy)
	fmt.Printf("Dry run: %v\n\n", fo.dryRun)

	if fo.recursive {
		return fo.organizeRecursive(fo.sourceDir)
	}
	return fo.organizeDirectory(fo.sourceDir)
}

func (fo *FileOrganizer) organizeRecursive(dir string) error {
	return filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		
		if info.IsDir() {
			return nil
		}
		
		return fo.processFile(path, info)
	})
}

func (fo *FileOrganizer) organizeDirectory(dir string) error {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return fmt.Errorf("failed to read directory: %w", err)
	}

	for _, entry := range entries {
		if entry.IsDir() {
			continue
		}

		path := filepath.Join(dir, entry.Name())
		info, err := entry.Info()
		if err != nil {
			log.Printf("Warning: failed to get info for %s: %v\n", path, err)
			continue
		}

		if err := fo.processFile(path, info); err != nil {
			log.Printf("Warning: failed to process %s: %v\n", path, err)
		}
	}

	return nil
}

func (fo *FileOrganizer) processFile(path string, info os.FileInfo) error {
	var targetSubDir string

	switch fo.organizeBy {
	case "type":
		targetSubDir = fo.getCategoryByExtension(filepath.Ext(path))
	case "date":
		targetSubDir = fo.getDateFolder(info.ModTime())
	case "extension":
		ext := filepath.Ext(path)
		if ext != "" {
			targetSubDir = strings.TrimPrefix(ext, ".")
		} else {
			targetSubDir = "no-extension"
		}
	default:
		return fmt.Errorf("unknown organize method: %s", fo.organizeBy)
	}

	targetDir := filepath.Join(fo.targetDir, targetSubDir)
	targetPath := filepath.Join(targetDir, filepath.Base(path))

	//skip if source and target are the same
	if path == targetPath {
		fo.skippedCount++
		return nil
	}

	if fo.dryRun {
		fmt.Printf("[DRY RUN] Would move: %s -> %s\n", path, targetPath)
		fo.movedCount++
		return nil
	}

	//create target directory if it doesn't exist
	if err := os.MkdirAll(targetDir, 0755); err != nil {
		return fmt.Errorf("failed to create directory %s: %w", targetDir, err)
	}

	//handle file name conflicts
	targetPath = fo.resolveConflict(targetPath)

	//move the file
	if err := os.Rename(path, targetPath); err != nil {
		return fmt.Errorf("failed to move file: %w", err)
	}

	fmt.Printf("Moved: %s -> %s\n", path, targetPath)
	fo.movedCount++
	return nil
}

func (fo *FileOrganizer) getCategoryByExtension(ext string) string {
	ext = strings.ToLower(ext)
	if category, ok := extensionCategories[ext]; ok {
		return category
	}
	return "Others"
}

func (fo *FileOrganizer) getDateFolder(t time.Time) string {
	return t.Format("2006-01")
}

func (fo *FileOrganizer) resolveConflict(path string) string {
	if _, err := os.Stat(path); os.IsNotExist(err) {
		return path
	}

	ext := filepath.Ext(path)
	nameWithoutExt := strings.TrimSuffix(path, ext)
	
	counter := 1
	for {
		newPath := fmt.Sprintf("%s_%d%s", nameWithoutExt, counter, ext)
		if _, err := os.Stat(newPath); os.IsNotExist(err) {
			return newPath
		}
		counter++
	}
}

func (fo *FileOrganizer) PrintSummary() {
	fmt.Printf("\n--- Summary ---\n")
	fmt.Printf("Files moved: %d\n", fo.movedCount)
	fmt.Printf("Files skipped: %d\n", fo.skippedCount)
	fmt.Printf("Total processed: %d\n", fo.movedCount+fo.skippedCount)
}

func main() {
	sourceDir := flag.String("source", ".", "source directory to organize")
	targetDir := flag.String("target", "./organized", "target directory for organized files")
	organizeBy := flag.String("by", "type", "organize by: type, date, or extension")
	dryRun := flag.Bool("dry-run", false, "preview changes without moving files")
	recursive := flag.Bool("recursive", false, "process subdirectories recursively")
	
	flag.Parse()

	//validate organize method
	validMethods := map[string]bool{"type": true, "date": true, "extension": true}
	if !validMethods[*organizeBy] {
		log.Fatal("Invalid organize method. Use: type, date, or extension")
	}

	//validate source directory
	if _, err := os.Stat(*sourceDir); os.IsNotExist(err) {
		log.Fatalf("Source directory does not exist: %s", *sourceDir)
	}

	organizer := NewFileOrganizer(*sourceDir, *targetDir, *organizeBy, *dryRun, *recursive)
	
	if err := organizer.Organize(); err != nil {
		log.Fatalf("Error organizing files: %v", err)
	}

	organizer.PrintSummary()
}