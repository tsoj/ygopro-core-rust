//! GranSolfachord Coolia
use super::prelude::*;

pub const ID: u32 = 84521924;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SOLFACHORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 2+ monsters, including a Pendulum Monster
    // Link.AddProcedure(c,nil,2,3,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(3), Some(group_check(lcheck)), None, None);
    // Gains 100 ATK for each face-up Pendulum Monster in the Extra Deck
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    card::register_effect(d, c, e1, false);
    // Activated effects of Pendulum Monsters this card points to cannot be negated
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_DISEFFECT);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_value_fn(d, e2, lua_fn(effval));
    card::register_effect(d, c, e2, false);
    let e2a = effect::create_effect(d, c);
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2a, EVENT_CHAINING);
    effect::set_range(d, e2a, LOCATION_MZONE);
    effect::set_label_object(d, e2a, Value::Effect(e2));
    effect::set_operation(d, e2a, Some(operation(checkop)));
    card::register_effect(d, c, e2a, false);
    // Special Summon 1 "Solfachord" card with an odd Pendulum Scale
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_NEGATE + CATEGORY_TOEXTRA);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_code(d, e3, EVENT_CHAINING);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
// function s.lcheck(g,lc,sumtype,tp)
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsType,1,nil,TYPE_PENDULUM,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_type_ctx(d, c, TYPE_PENDULUM, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsType,TYPE_PENDULUM),e:GetHandlerPlayer(),LOCATION_EXTRA,0,nil)*100
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM)), p, LOCATION_EXTRA, 0, NO_EX) as i64 * 100
}
/// The Lua table `effs` (`e2`'s label object), kept as `Value::Multi` of `(effect field id, card
/// field id)` pairs.
fn effs_get(d: &Duel, e2: EffectId, key: i64) -> Option<i64> {
    match effect::get_label_object(d, e2) {
        Value::Multi(v) => v.chunks(2).find(|p| p[0].int() == key).map(|p| p[1].int()),
        _ => None,
    }
}
// function s.effval(e,ct)
fn effval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // local te=Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_EFFECT)
    let te = duel::get_chain_info(d, a.int(1) as u8, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
    // local effs=e:GetLabelObject()
    // return effs and effs[te:GetFieldID()]==e:GetHandler():GetFieldID()
    if !matches!(effect::get_label_object(d, e), Value::Multi(_)) {
        return false;
    }
    let key = effect::get_field_id(d, te) as i64;
    let h = effect::get_handler(d, e).nil();
    let fid = card::get_field_id(d, h) as i64;
    effs_get(d, e, key) == Some(fid)
}
// function s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // local e2=e:GetLabelObject()
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    // if Duel.GetCurrentChain()==1 then e2:SetLabelObject({}) end
    if duel::get_current_chain(d, false) == 1 {
        effect::set_label_object(d, e2, Value::Multi(vec![]));
    }
    // if re:IsActiveType(TYPE_PENDULUM) and c:GetLinkedGroup():IsContains(re:GetHandler()) then
    //     e2:GetLabelObject()[re:GetFieldID()]=c:GetFieldID()
    let re = a.re.nil();
    if effect::is_active_type(d, re, TYPE_PENDULUM) {
        let lg = card::get_linked_group(d, c);
        let rc = effect::get_handler(d, re);
        if rc.is_some_and(|rc| lg.contains(rc)) {
            let key = effect::get_field_id(d, re) as i64;
            let val = card::get_field_id(d, c) as i64;
            let Value::Multi(mut v) = effect::get_label_object(d, e2) else {
                crate::lua_error("attempt to index a nil value");
            };
            match v.chunks(2).position(|p| p[0].int() == key) {
                Some(i) => v[2 * i + 1] = Value::Int(val),
                None => {
                    v.push(Value::Int(key));
                    v.push(Value::Int(val));
                }
            }
            effect::set_label_object(d, e2, Value::Multi(v));
        }
    }
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and rp==1-tp and Duel.IsChainNegatable(ev)
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED) && a.rp == 1 - a.tp && duel::is_chain_negatable(d, a.ev as u8)
}
// function s.spfilter(c,e,tp,zone)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsSetCard(SET_SOLFACHORD) and c:IsOddScale()
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16])
        && card::is_odd_scale(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // local zone=e:GetHandler():GetLinkedZone(tp)
        let h = effect::get_handler(d, a.e).nil();
        let zone = card::get_linked_zone(d, h, Some(tp));
        // return Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)>0
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_PZONE,0,1,nil,e,tp,zone)
        let e = a.e;
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, zone)), tp, LOCATION_PZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_PZONE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_PZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOEXTRA,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.tefilter(c)
fn tefilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SOLFACHORD) and c:IsType(TYPE_PENDULUM) and c:IsEvenScale() and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::is_type(d, c, TYPE_PENDULUM) && card::is_even_scale(d, c) && !card::is_forbidden(d, c)
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local zone=e:GetHandler():GetLinkedZone(tp)
    let zone = card::get_linked_zone(d, c, Some(tp));
    // if Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)==0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0 == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_PZONE,0,1,1,nil,e,tp,zone)
    let sg = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp, zone)), tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX).nil();
    // if #sg==0 or Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP,zone)==0 or not Duel.NegateActivation(ev) then return end
    if sg.is_empty() || duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, Some(zone)) == 0 || !duel::negate_activation(d, a.ev as u8) {
        return;
    }
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsRelateToEffect(re) then
    //     Duel.SendtoGrave(eg,REASON_EFFECT)
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && card::is_relate_to_effect(d, rc, re) {
        duel::send_to_grave(d, a.eg(), REASON_EFFECT, None, None);
    }
    // local g=Duel.GetMatchingGroup(s.tefilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| tefilter(d, x)), tp, LOCATION_DECK, 0, NO_EX);
    // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
    if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELECT)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELECT as u64);
    // local pg=g:Select(tp,1,1,nil)
    let pg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    // if #pg>0 then Duel.BreakEffect() Duel.SendtoExtraP(pg,tp,REASON_EFFECT) end
    if !pg.is_empty() {
        duel::break_effect(d);
        duel::send_to_extra_p(d, pg, Some(tp), REASON_EFFECT, None);
    }
}
