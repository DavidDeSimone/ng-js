(require 'ng-js)
(require 'filenotify)

(setq ng-js-temp (make-temp-file "ng-js"))
(ng-js-eval (format "ngjsSetFileMarker('%s')" ng-js-temp))
(defun execute-lisp (event)
  (ng-js-drain))

(setq desc (file-notify-add-watch
  ng-js-temp '(change attribute-change) 'execute-lisp))