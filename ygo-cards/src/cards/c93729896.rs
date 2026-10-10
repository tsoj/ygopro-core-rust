//! Nightmare Throne
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 93729896;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_YUBEL as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Keep track of the "Yubel" monsters that left the field
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2a:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2a, EVENT_LEAVE_FIELD);
    // e2a:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2a, LOCATION_FZONE);
    // e2a:SetCondition(s.regcon)
    effect::set_condition(d, e2a, Some(condition(regcon)));
    // e2a:SetOperation(s.regop)
    effect::set_operation(d, e2a, Some(operation(regop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // Add 1 "Yubel" monster to your hand
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2b, aux::stringid(ID, 1));
    // e2b:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2b, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON);
    // e2b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2b:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2b, EFFECT_FLAG_DELAY, 0);
    // e2b:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2b, EVENT_CUSTOM + ID);
    // e2b:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2b, LOCATION_FZONE);
    // e2b:SetCountLimit(1)
    effect::set_count_limit(d, e2b, 1, 0, 0, 0);
    // e2b:SetCondition(function(e,tp,eg,ep) return ep==tp end)
    effect::set_condition(d, e2b, Some(condition(ep_is_tp)));
    // e2b:SetTarget(s.thtg)
    effect::set_target(d, e2b, Some(cost_target(thtg)));
    // e2b:SetOperation(s.thop)
    effect::set_operation(d, e2b, Some(operation(thop)));
    // e2b:SetLabelObject(e2a)
    effect::set_label_object(d, e2b, Value::Effect(e2a));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `function(e,tp,eg,ep) return ep==tp end`
fn ep_is_tp(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == a.tp
}
/// `s.thdesfilter(c)`
fn thdesfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FIEND) and c:IsAttack(0) and c:IsDefense(0)
    card::is_race(d, c, RACE_FIEND) && card::is_attack(d, c, &[0]) && card::is_defense(d, c, &[0])
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.thdesfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thdesfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
        // local sc=g:Select(tp,1,1,nil):GetFirst()
        let selected = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &selected) else {
            return;
        };
        // aux.ToHandOrElse(sc,tp,nil,function() Duel.Destroy(sc,REASON_EFFECT) end,aux.Stringid(id,3))
        let oper = |d: &mut Duel, _g: &CardSet| -> i32 {
            duel::destroy(d, sc, REASON_EFFECT, None, None);
            0
        };
        aux::to_hand_or_else(d, sc, tp, None, Some(&oper), Some(aux::stringid(ID, 3)));
    }
}
/// `s.thfilter(c,tp,levels)`
fn thfilter(d: &mut Duel, c: CardId, levels: &[i64]) -> bool {
    // if not (c:IsSetCard(SET_YUBEL) and (c:IsFaceup() or not c:IsLocation(LOCATION_REMOVED)) and c:IsAbleToHand() and c:HasLevel()) then return false end
    if !(card::is_set_card(d, c, &[SET_YUBEL as u16])
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_REMOVED))
        && card::is_able_to_hand(d, c, None)
        && card::has_level(d, c))
    {
        return false;
    }
    // for _,lv in ipairs(levels) do
    for &lv in levels {
        // if c:IsOriginalLevel(lv-1) or c:IsOriginalLevel(lv+1) then return true end
        if (lv - 1 >= 0 && card::is_original_level(d, c, (lv - 1) as u32)) || (lv + 1 >= 0 && card::is_original_level(d, c, (lv + 1) as u32)) {
            return true;
        }
    }
    // return false
    false
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local levels={e:GetLabelObject():GetLabel()}
    let e2a = effect::get_label_object(d, e).as_effect().nil();
    let levels = effect::get_labels(d, e2a);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_REMOVED|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,tp,levels) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, &levels)), tp, LOCATION_REMOVED | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // e:SetLabel(table.unpack(levels))
    effect::set_label(d, e, &levels);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_REMOVED|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_REMOVED | LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_REMOVED|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,tp,{e:GetLabel()}):GetFirst()
    let levels = vec![effect::get_label(d, e)];
    let selected = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| thfilter(d, c, &levels))),
        tp,
        LOCATION_REMOVED | LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &selected) else {
        return;
    };
    // if sc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) then Duel.HintSelection(sc,true) end
    if card::is_location(d, sc, LOCATION_GRAVE | LOCATION_REMOVED) {
        duel::hint_selection(d, sc, true);
    }
    // if not (Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND)) then return end
    if !(duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_HAND)) {
        return;
    }
    // Duel.ConfirmCards(1-tp,sc)
    duel::confirm_cards(d, 1 - tp, sc);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and sc:IsCanBeSpecialSummoned(e,0,tp,true,false)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,4)) then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, sc, e, 0, tp, true, false, None, None, None)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 4))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(sc,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, sc, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    }
}
/// `s.thconfilter(c,tp)`
fn thconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousSetCard(SET_YUBEL) and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:IsPreviousPosition(POS_FACEUP) and c:IsReason(REASON_EFFECT) and c:HasLevel()
    card::is_previous_set_card(d, c, &[SET_YUBEL as u16])
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP as u32)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::has_level(d, c)
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.thconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| thconfilter(d, c, tp), 1, NO_EX)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let (e, tp, re, r, rp, ev) = (a.e, a.tp, a.re, a.r, a.rp, a.ev);
    // local g=eg:Filter(s.thconfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| thconfilter(d, c, tp), NO_EX);
    // if not Duel.IsChainSolving() then
    if !duel::is_chain_solving(d) {
        // Duel.RaiseEvent(g,EVENT_CUSTOM+id,re,r,rp,tp,ev)
        duel::raise_event(d, g, EVENT_CUSTOM + ID, re, r, rp, tp, ev);
    } else {
        // local label_obj=e:GetLabelObject()
        // if label_obj then
        if let Some(mut label_obj) = effect::get_label_object(d, e).as_group().cloned() {
            // label_obj:Merge(g)
            group::add_card(&mut label_obj, &g);
            // local levels={e:GetLabel()}
            let mut levels = vec![effect::get_label(d, e)];
            // for tc in g:Iter() do table.insert(levels,tc:GetOriginalLevel()) end
            for tc in group::iter(d, &g) {
                levels.push(card::get_original_level(d, tc) as i64);
            }
            // e:SetLabel(table.unpack(levels))
            effect::set_label(d, e, &levels);
            // (the group is stored back: the label object is the same group in Lua)
            effect::set_label_object(d, e, Value::Group(label_obj));
        } else {
            // e:SetLabel(table.unpack(g:GetClass(Card.GetOriginalLevel)))
            let classes = group::get_class(d, &g, &|d, c| card::get_original_level(d, c) as i64);
            effect::set_label(d, e, &classes);
            // e:SetLabelObject(g)
            effect::set_label_object(d, e, Value::Group(g.clone()));
            // local c=e:GetHandler()
            let c = effect::get_handler(d, e).nil();
            // Raise the custom event at the end of the Chain
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EVENT_CHAIN_SOLVED)
            effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
            // e1:SetRange(LOCATION_FZONE)
            effect::set_range(d, e1, LOCATION_FZONE);
            // e1:SetCondition(function() return Duel.GetCurrentChain()==1 end)
            effect::set_condition(d, e1, Some(lua_fn(|d, _a: Args| duel::get_current_chain(d, false) == 1)));
            // e1:SetOperation(function(eff)
            //     local g=e:GetLabelObject()
            //     eff:Reset()
            //     e:SetLabelObject(nil)
            //     if g then Duel.RaiseEvent(g,EVENT_CUSTOM+id,re,r,rp,tp,ev) end
            // end)
            effect::set_operation(
                d,
                e1,
                Some(lua_fn(move |d, a: Args| {
                    let eff = a.e(0);
                    let g = effect::get_label_object(d, e);
                    effect::reset(d, eff);
                    effect::set_label_object(d, e, Value::Nil);
                    if let Value::Group(g) = g {
                        duel::raise_event(d, g, EVENT_CUSTOM + ID, re, r, rp, tp, ev);
                    }
                })),
            );
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EVENT_CHAIN_NEGATED)
            effect::set_code(d, e2, EVENT_CHAIN_NEGATED);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
        }
    }
}
