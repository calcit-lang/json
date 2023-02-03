
{} (:package |json)
  :configs $ {} (:init-fn |json.test/main!) (:reload-fn |json.test/reload!) (:version |0.0.1)
    :modules $ []
  :entries $ {}
  :files $ {}
    |json.core $ {}
      :defs $ {}
        |parse $ quote
          defn parse (content)
            &call-dylib-edn (get-dylib-path "\"/dylibs/libcalcit_std") "\"json_parse" content
        |stringify $ quote
          defn stringify (data ? pretty?)
            &call-dylib-edn (get-dylib-path "\"/dylibs/libcalcit_std") "\"json_stringify" data pretty?
      :ns $ quote
        ns json.core $ :require
          json.$meta :refer $ calcit-dirname
          json.util :refer $ get-dylib-path
    |json.test $ {}
      :defs $ {}
        |main! $ quote
          defn main! () $ run-tests
        |reload! $ quote
          defn reload! $
        |run-tests $ quote
          defn run-tests () (println "\"%%%% test for json") (println calcit-filename calcit-dirname)
            println "\"Parsing:" $ parse "\"{\"a\":[\"b\",1]}"
            println "\"Stringify:" $ stringify
              {} $ :a 1
      :ns $ quote
        ns json.test $ :require
          json.core :refer $ parse stringify
          json.$meta :refer $ calcit-dirname calcit-filename
    |json.util $ {}
      :defs $ {}
        |get-dylib-ext $ quote
          defmacro get-dylib-ext () $ case-default (&get-os) "\".so" (:macos "\".dylib") (:windows "\".dll")
        |get-dylib-path $ quote
          defn get-dylib-path (p)
            str (or-current-path calcit-dirname) p $ get-dylib-ext
        |or-current-path $ quote
          defn or-current-path (p)
            if (blank? p) "\"." p
      :ns $ quote
        ns json.util $ :require
          json.$meta :refer $ calcit-dirname calcit-filename
