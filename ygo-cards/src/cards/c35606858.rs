//! Masterking Archfiend
use super::prelude::*;

pub const ID: u32 = 35606858;
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_FIEND),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_FIEND, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // Maintenance cost
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(_,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e1:SetOperation(s.mtop)
    effect::set_operation(d, e1, Some(operation(mtop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send 1 Fiend from your Deck to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_PAY_LPCOST)
    effect::set_code(d, e2, EVENT_PAY_LPCOST);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e2:SetCondition(function(_,tp,_,ep) return ep==tp end)
    effect::set_condition(d, e2, Some(condition(|_d, a| a.ep == a.tp)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Roll a die and apply the appropriate result
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DICE+CATEGORY_TOHAND+CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_DICE + CATEGORY_TOHAND + CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e3, EVENT_CUSTOM + ID);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e3, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e3:SetTarget(s.dctg)
    effect::set_target(d, e3, Some(cost_target(dctg)));
    // e3:SetOperation(s.dcop)
    effect::set_operation(d, e3, Some(operation(dcop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e3:SetLabelObject(g)
    // (the group is kept as the label object of e3; the core's KeepAlive does nothing here)
    effect::set_label_object(d, e3, Value::Group(CardSet::new()));
    // Register Fiend monsters sent to your GY
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3a:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3a, EVENT_TO_GRAVE);
    // e3a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3a, LOCATION_MZONE);
    // e3a:SetLabelObject(e3)
    effect::set_label_object(d, e3a, Value::Effect(e3));
    // e3a:SetOperation(s.regop)
    effect::set_operation(d, e3a, Some(operation(regop)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
}
/// `s.mtop(e,tp,eg,ep,ev,re,r,rp)`
fn mtop(d: &mut Duel, a: &EffArgs) {
    // if Duel.CheckLPCost(tp,500) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::check_lp_cost(d, a.tp, 500) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        // Duel.PayLPCost(tp,500)
        duel::pay_lp_cost(d, a.tp, 500);
    } else {
        // Duel.Destroy(e:GetHandler(),REASON_COST)
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_COST, None, None);
    }
}
/// `s.tgfilter(c,val)`
fn tgfilter(d: &mut Duel, c: CardId, val: u32) -> bool {
    // return c:IsRace(RACE_FIEND) and c:IsMonster() and (c:IsAttack(val) or c:IsDefense(val)) and c:IsAbleToGrave()
    card::is_race(d, c, RACE_FIEND)
        && card::is_monster(d, c)
        && (card::is_attack(d, c, &[val]) || card::is_defense(d, c, &[val]))
        && card::is_able_to_grave(d, c, None)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil,ev) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| tgfilter(d, c, a.ev)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil,ev)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| tgfilter(d, c, a.ev)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.cfilter(c,e,tp,ft)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return c:IsRace(RACE_FIEND) and c:IsControler(tp) and (c:IsAbleToHand() or c:IsAbleToDeck()
    //     or (ft>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false))) and c:HasFlagEffect(id+1)
    card::is_race(d, c, RACE_FIEND)
        && card::is_controler(d, c, tp)
        && (card::is_able_to_hand(d, c, None)
            || card::is_able_to_deck(d, c)
            || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
        && card::has_flag_effect(d, c, ID + 1, None)
}
/// `s.regfilter(c,tp)`
fn regfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_FIEND) and c:IsControler(tp)
    card::is_race(d, c, RACE_FIEND) && card::is_controler(d, c, tp)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local tg=eg:Filter(s.regfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| regfilter(d, c, a.tp), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
            // tc:RegisterFlagEffect(id+1,RESET_EVENT|RESET_REMOVE|RESET_TEMP_REMOVE|RESET_TOHAND|RESET_TODECK|RESET_LEAVE|RESET_TOFIELD,0,1)
            card::register_flag_effect(
                d,
                tc,
                ID + 1,
                RESET_EVENT | RESET_REMOVE | RESET_TEMP_REMOVE | RESET_TOHAND | RESET_TODECK | RESET_LEAVE | RESET_TOFIELD,
                0,
                1,
                0,
                0,
            );
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e3 = effect::get_label_object(d, a.e).as_effect().nil();
        let mut g = effect::get_label_object(d, e3).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e3, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        let h = effect::get_handler(d, a.e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, a.tp, 0);
    }
}
/// `s.dctg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dctg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local g=e:GetLabelObject():Filter(s.cfilter,nil,e,tp,ft)
    let lg = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let g = group::filter(d, &lg, &|d, c| cfilter(d, c, a.e, a.tp, ft), NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.mon_select(g,tp,func,hint,...)`
fn mon_select(d: &mut Duel, g: &CardSet, tp: u8, func: &dyn Fn(&mut Duel, CardId) -> bool, hint: u64) -> Option<CardId> {
    // if #g>1 then
    let tc = if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,hint)
        duel::hint(d, HINT_SELECTMSG as u8, tp, hint);
        // tc=g:FilterSelect(tp,func,1,1,nil,...):GetFirst()
        let sel = group::filter_select(d, g, tp, func, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel)
    } else {
        // tc=g:GetFirst()
        // if not func(tc,...) then tc=nil end
        group::get_first(d, g).filter(|&x| func(d, x))
    };
    // if tc then Duel.HintSelection(tc,true) end
    if let Some(x) = tc {
        duel::hint_selection(d, CardSet::from_card(x), true);
    }
    // return tc
    tc
}
/// `s.dcop(e,tp,eg,ep,ev,re,r,rp)`
fn dcop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local g=e:GetLabelObject():Filter(s.cfilter,nil,e,tp,ft)
    let lg = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let g = group::filter(d, &lg, &|d, c| cfilter(d, c, a.e, a.tp, ft), NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local d=Duel.TossDice(tp,1)
    let dice = duel::toss_dice(d, a.tp, 1, 0)[0];
    if dice == 1 {
        // tc=s.mon_select(g,tp,Card.IsAbleToHand,HINTMSG_ATOHAND)
        let Some(tc) = mon_select(d, &g, a.tp, &|d, c| card::is_able_to_hand(d, c, None), HINTMSG_ATOHAND as u64) else {
            return;
        };
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(tc));
    } else if dice == 6 {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
            return;
        }
        // tc=s.mon_select(g,tp,Card.IsCanBeSpecialSummoned,HINTMSG_SPSUMMON,e,0,tp,false,false)
        let Some(tc) = mon_select(
            d,
            &g,
            a.tp,
            &|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None),
            HINTMSG_SPSUMMON as u64,
        ) else {
            return;
        };
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    } else if dice > 1 && dice < 6 {
        // tc=s.mon_select(g,tp,Card.IsAbleToDeck,HINTMSG_TODECK)
        let Some(tc) = mon_select(d, &g, a.tp, &|d, c| card::is_able_to_deck(d, c), HINTMSG_TODECK as u64) else {
            return;
        };
        // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
