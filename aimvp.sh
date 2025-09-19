# shellcheck shell=zsh
# ==============================================================================
# aimvp: 格式化 Markdown 字串
# 功能：
# - 接收字串輸入，並進行一系列正規化處理
#
# 規則：
# 1. 將行首的無序列表符號 `*` 換成 `-`
# 2. 將列表標記 (`-` 或 `1.`) 後的多個空格，統一為一個
# 3. 移除所有列表行中的粗體標記 `**`
# 4. 移除所有標題行 (`#`) 中的粗體標記 `**`
# ==============================================================================
aimvp() {
  # 如果 Homebrew 已安裝 aimvp 的話便直接呼叫
  if brew --prefix aimvp >/dev/null 2>&1; then
    command aimvp "$@"
    return
  fi

  # 處理選項
  case "$1" in
    -h|--help|"")
      local help_message="
用法: aimvp <選項> <檔案路徑>

高效的 Markdown 格式化工具

選項:
  -h, --help         顯示此說明訊息
  -f, --file <FILE>  讀取指定的 Markdown 檔案進行處理，
                     並生成一個帶有 .mvp.md 後綴的新檔案。

範例:
  aimvp -f README.md
  aimvp --file README.md
"
      echo -e "$help_message"
      # 如果是明確請求 help，回傳成功(0)；否則是因無效輸入，回傳錯誤(1)
      if [[ "$1" == "-h" || "$1" == "--help" ]]; then
        return 0
      else
        return 1
      fi
      ;;
    -f|--file)
      # --- 檔案處理模式 ---
      local input_file="$2"
      # 檢查是否提供了檔案名稱
      if [[ -z "$input_file" ]]; then
        echo "錯誤: 選項 '$1' 後面需要提供一個檔案路徑。" >&2
        aimvp --help # 提示使用者正確用法
        return 1
      fi
      # 檢查檔案是否存在
      if [[ ! -f "$input_file" ]]; then
        echo "錯誤: 找不到檔案 '$input_file'" >&2
        return 1
      fi
      # 自動生成輸出檔名
      local base_name="${input_file%.md}"
      local output_file="${base_name}.mvp.md"
      # 防呆機制：避免重複處理
      if [[ "$input_file" == "$output_file" ]]; then
        echo "錯誤: 輸入檔案和預計生成的輸出檔案名稱相同。" >&2
        echo "提示: 請不要對 .mvp.md 檔案重複執行此腳本。" >&2
        return 1
      fi
      # 核心 sed 處理 (輸入為檔案，輸出到新檔案)
      sed -E \
        -e 's/^([[:space:]]*)\*[[:space:]]/\1- /' \
        -e 's/^([[:space:]]*(-|[0-9]+\.))[[:space:]]+/\1 /' \
        -e '/^[[:space:]]*(-|[0-9]+\.)/ s/\*\*([^*]*)\*\*/\1/g' \
        -e '/^#{1,6}[[:space:]]/ s/\*\*([^*]*)\*\*/\1/g' \
        "$input_file" > "$output_file"

      # pangu
      if command -v npx &> /dev/null; then
        npx pangu -f "$output_file" > "$output_file" || echo "⚠️ pangu 執行失敗，已跳過"
      else
        echo "⚠️ npx 不存在，跳過 pangu"
      fi

      # prettier
      if command -v prettier &> /dev/null; then
        prettier -w "$output_file" > /dev/null || echo "⚠️ prettier 執行失敗，已跳過"
      else
        echo "⚠️ prettier 不存在，跳過格式化"
      fi
      echo ""
      echo "  ✔ 處理完成"
      echo "  -> $output_file"
      return 0
      ;;
  esac
}
