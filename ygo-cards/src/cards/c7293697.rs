//! Toon World the Perfect World
//! 完全なる世界 トゥーン・ワールド
use super::prelude::*;

pub const ID: u32 = 7293697;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_TOON_WORLD], listed_series: &[SET_TOON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);

    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetValue(CARD_TOON_WORLD)
    effect::set_value(d, e1, CARD_TOON_WORLD as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(3,id)
    effect::set_count_limit(d, e2, 3, ID, 0, 0);
    // e2:SetCost(s.thcost)
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);

    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCondition(s.rmcon)
    effect::set_condition(d, e3, Some(condition(rmcon)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);

    // aux.GlobalCheck(s,function()
    //     s.name_list={}
    //     s.name_list[0]={}
    //     s.name_list[1]={}
    //     aux.AddValuesReset(function()
    //         s.name_list[0]={}
    //         s.name_list[1]={}
    //     end)
    // end)
    aux::global_check(d, ID, |d| {
        set_name_list(d, 0, Vec::new());
        set_name_list(d, 1, Vec::new());
        aux::add_values_reset(d, reset_name_list);
    });
}
/// `s.name_list[p]`: the original codes banished by this card's effect this turn, stored as the
/// script variable of the player (a list of integers)
fn name_list(d: &Duel, p: u8) -> Vec<i64> {
    match s_var(d, ID, p as u64) {
        Value::Multi(v) => v.iter().filter_map(|x| if let Value::Int(i) = x { Some(*i) } else { None }).collect(),
        _ => Vec::new(),
    }
}
fn set_name_list(d: &mut Duel, p: u8, list: Vec<i64>) {
    set_s_var(d, ID, p as u64, Value::Multi(list.into_iter().map(Value::Int).collect()));
}
/// `aux.AddValuesReset(function() s.name_list[0]={} s.name_list[1]={} end)`
fn reset_name_list(d: &mut Duel) {
    set_name_list(d, 0, Vec::new());
    set_name_list(d, 1, Vec::new());
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:HasFlagEffect(id) end
    if a.chk == Some(0) {
        return !card::has_flag_effect(d, c, ID, None);
    }
    // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_TOON) or c:ListsCodeWithArchetype(SET_TOON)) and c:IsAbleToHand()
    (card::is_set_card(d, c, &[SET_TOON as u16]) || card::lists_code_with_archetype(d, c, &[SET_TOON as u32])) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.rmfilter(c,tp)`
fn rmfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_TOON) and c:IsFaceup() and c:IsAbleToRemove() and not s.name_list[tp][c:GetOriginalCodeRule()]
    let code = card::get_original_code_rule(d, c).first().copied().unwrap_or(0) as i64;
    card::is_type(d, c, TYPE_TOON)
        && card::is_faceup(d, c)
        && card::is_able_to_remove(d, c, None, Some(POS_FACEUP as u8), Some(REASON_EFFECT))
        && !name_list(d, tp).contains(&code)
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return re:IsActivated() and re:GetHandler()~=e:GetHandler()
    //     and Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_MZONE,0,1,nil,tp)
    let re = a.re.nil();
    effect::is_activated(d, re)
        && effect::get_handler(d, re) != effect::get_handler(d, a.e)
        && duel::is_existing_matching_card(d, Some(&|d, c| rmfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| rmfilter(d, c, tp)), tp, LOCATION_MZONE, 0, NO_EX);
    let handler = effect::get_handler(d, a.e).nil();
    // if #g>0 and Duel.SelectEffectYesNo(tp,e:GetHandler(),aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_effect_yes_no(d, tp, handler, Some(aux::stringid(ID, 1))) {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local sc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // if sc then
        if let Some(sc) = sc {
            // Duel.HintSelection(sc)
            duel::hint_selection(d, sc, true);
            // local temp_banish_eff=aux.RemoveUntil(sc,nil,REASON_EFFECT,PHASE_END,id,e,tp,aux.DefaultFieldReturnOp,nil,RESET_CHAIN)
            let temp_banish_eff = aux::remove_until(
                d,
                sc,
                None,
                REASON_EFFECT,
                PHASE_END,
                ID,
                a.e,
                tp,
                Some(aux::default_field_return()),
                None,
                Some(RESET_CHAIN),
                None,
                None,
                None,
            )
            .nil();
            // local e1=temp_banish_eff:Clone()
            let e1 = effect::clone(d, temp_banish_eff);
            // e1:SetCode(EVENT_CHAIN_SOLVED)
            effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
            // temp_banish_eff:Reset()
            effect::reset(d, temp_banish_eff);
            // s.name_list[tp][sc:GetOriginalCodeRule()]=true
            let code = card::get_original_code_rule(d, sc).first().copied().unwrap_or(0) as i64;
            let mut list = name_list(d, tp);
            if !list.contains(&code) {
                list.push(code);
            }
            set_name_list(d, tp, list);
        }
    }
}
