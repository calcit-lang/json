
{} (:package |json)
  :configs $ {} (:init-fn |json.test/main!) (:reload-fn |json.test/reload!) (:version |0.0.6)
    :modules $ []
  :entries $ {}
  :files $ {}
    |json.core $ %{} :FileEntry
      :defs $ {}
        |parse $ %{} :CodeEntry (:doc |)
          :code $ quote
            defn parse (content)
              &call-dylib-edn (get-dylib-path "\"/dylibs/libcalcit_std") "\"json_parse" content
        |stringify $ %{} :CodeEntry (:doc |)
          :code $ quote
            defn stringify (data ? pretty?)
              &call-dylib-edn (get-dylib-path "\"/dylibs/libcalcit_std") "\"json_stringify" data pretty?
      :ns $ %{} :CodeEntry (:doc |)
        :code $ quote
          ns json.core $ :require
            json.$meta :refer $ calcit-dirname
            json.util :refer $ get-dylib-path
    |json.test $ %{} :FileEntry
      :defs $ {}
        |main! $ %{} :CodeEntry (:doc |)
          :code $ quote
            defn main! () $ run-tests
        |reload! $ %{} :CodeEntry (:doc |)
          :code $ quote
            defn reload! $
        |run-tests $ %{} :CodeEntry (:doc |)
          :code $ quote
            defn run-tests () (println "\"%%%% test for json") (println calcit-filename calcit-dirname)
              println "\"Parsing:" $ parse "\"{\"a\":[\"b\",1]}"
              println "\"Stringify:" $ stringify
                {} $ :a 1
      :ns $ %{} :CodeEntry (:doc |)
        :code $ quote
          ns json.test $ :require
            json.core :refer $ parse stringify
            json.$meta :refer $ calcit-dirname calcit-filename
    |json.util $ %{} :FileEntry
      :defs $ {}
        |get-dylib-ext $ %{} :CodeEntry (:doc |)
          :code $ quote
            defmacro get-dylib-ext () $ case-default (&get-os) "\".so" (:macos "\".dylib") (:windows "\".dll")
        |get-dylib-path $ %{} :CodeEntry (:doc |)
          :code $ quote
            defn get-dylib-path (p)
              str (or-current-path calcit-dirname) p $ get-dylib-ext
        |or-current-path $ %{} :CodeEntry (:doc |)
          :code $ quote
            defn or-current-path (p)
              if (blank? p) "\"." p
      :ns $ %{} :CodeEntry (:doc |)
        :code $ quote
          ns json.util $ :require
            json.$meta :refer $ calcit-dirname calcit-filename
