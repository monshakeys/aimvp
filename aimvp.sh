# shellcheck shell=zsh
# ==============================================================================
# aimvp: AI Markdown Visual Presentation
# ==============================================================================
aimvp() {
  # 如果 Homebrew 已安裝 aimvp 的話便直接呼叫
  if brew --prefix aimvp >/dev/null 2>&1; then
    command aimvp "$@"
    return
  fi

  # --- 參數與選項處理 ---
  if [[ "$1" == "-h" || "$1" == "--help" || -z "$1" ]]; then
    local help_message="
用法: aimvp <檔案路徑>

針對 AI Agent 輸出的 markdown 格式檔案進行重新排版

1. 使用 Prettier 進行整體格式化
2. 使用 Pangu 在中英文之間添加空格
3. 移除標題列 (H1-H6) 內部多餘的粗體標記
4. 標準化標題列的空格
5. 移除清單項目開頭的粗體標記
6. 標準化清單項目的縮排與空格

選項:
  -h, --help  顯示此說明訊息
"
    echo -e "$help_message"
    # 如果是明確請求 help，回傳成功(0)；否則是因無效輸入，回傳錯誤(1)
    if [[ "$1" == "-h" || "$1" == "--help" ]]; then
      return 0
    else
      return 1
    fi
  fi

  local input_file="$1"

  # --- 輸入檔案驗證 ---
  if [[ ! -f "$input_file" ]]; then
    echo "❌ 錯誤：檔案 '$input_file' 不存在。" >&2
    return 1
  fi

  if [[ "${input_file:t:e}" != "md" ]]; then
    echo "❌ 錯誤：僅支援 .md 副檔名的 Markdown 檔案。" >&2
    return 1
  fi

  # --- 依賴工具檢查 ---
  local required_tools=(npx sponge ugrep sed)
  local missing_tool=0
  for tool in "${required_tools[@]}"; do
    if ! command -v "$tool" >/dev/null 2>&1; then
      echo "⚠️ 警告：必要指令 '$tool' 不存在，請先安裝。" >&2
      missing_tool=1
    fi
  done

  if [[ $missing_tool -eq 1 ]]; then
    echo "❌ 偵測到缺少必要工具，程式已終止。" >&2
    return 1
  fi

  # --- 核心處理邏輯 ---
  loading_with_status() {
    local chars="⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
    local delay=0.1
    local count=0
    local status_file="/tmp/__aimvp__status"
    local now_time=$(date '+%Y%m%d%H%M%S')
    local file_backup=${input_file%.*}.backup.${now_time}.md
    cp "$input_file" "$file_backup"

    # 關閉工作控制
    set +m
    echo "RUNNING" > "$status_file"
    # 背景執行
    (
      cat "$input_file" | \

        ugrep -P -y -e '^(\s*)\* +(.*)' --replace='%1- %2' | \
        sed -E '/^#/ s/\*\*//g' | \

        npx prettier --parser markdown | \
        { content=$(dd 2>/dev/null); npx pangu "$content"; } | \

        ugrep -P -y -e '^(\s*)- +\*\*([^*]+)\*\*(.*)' --replace='%1- %2%3' | \
        ugrep -P -y -e '^(\s*)([0-9]+)\. +\*\*([^*]+)\*\*(.*)' --replace='%1%2. %3%4' | \
        ugrep -P -y -e '^(\s*)- +(.+)' --replace='%1- %2' | \
        ugrep -P -y -e '^(\s*)([0-9]+)\. +(.+)' --replace='%1%2. %3' | \
        sponge "$input_file"

      echo "OK" > "$status_file"
    ) >/dev/null 2>&1 &
    local pid=$!
    # 動畫循環
    while [ "$(cat "$status_file" 2>/dev/null)" != "OK" ]; do
      printf "\r✨ 檔案格式化中 ${chars:$((count % ${#chars})):1}"
      sleep $delay
      ((count++))
    done
    # 等待完成並清除
    wait $pid
    printf "\r\033[K"
    # 重新啟用工作控制
    set -m
    rm -f "$status_file"
    echo "✅ 檔案 '$input_file' 已成功格式化"
  }
  loading_with_status
}
