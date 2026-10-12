//! The Stars of Raise Moon
use super::prelude::*;

pub const ID: u32 = 100458024;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_RAISE_MOON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place 1 "Raise Moon" card from your Deck on top of your Deck, except "The Stars of Raise Moon", but you cannot place another card with the same name with this effect of "The Stars of Raise Moon" this turn, also neither player can add cards from the Deck to the hand for the rest of this turn after this card resolves, except by drawing them
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish this card from your GY, then activate 1 of these effects;
    // ● Each player draws 1 card
    // ● If you control a Spellcaster "Raise Moon" Xyz Monster: Draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Keep track of the chosen cards' names
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // s.name_list={}; s.name_list[0]={}; s.name_list[1]={}
        set_name_list(d, 0, Vec::new());
        set_name_list(d, 1, Vec::new());
        // aux.AddValuesReset(function() s.name_list[0]={}; s.name_list[1]={} end)
        aux::add_values_reset(d, reset_name_list);
    });
}
/// The `s.name_list[p]` of this duel (the codes are kept as a list in a duel variable)
fn name_list(d: &Duel, p: u8) -> Vec<Value> {
    match s_var(d, ID, p as u64 + 1) {
        Value::Multi(v) => v,
        _ => Vec::new(),
    }
}
fn set_name_list(d: &mut Duel, p: u8, v: Vec<Value>) {
    set_s_var(d, ID, p as u64 + 1, Value::Multi(v));
}
/// `s.name_list[p][code]`
fn name_listed(d: &Duel, p: u8, code: u32) -> bool {
    name_list(d, p).iter().any(|x| x.int() == code as i64)
}
/// The reset of `s.name_list` at the end of each turn
fn reset_name_list(d: &mut Duel) {
    set_name_list(d, 0, Vec::new());
    set_name_list(d, 1, Vec::new());
}
/// `s.plfilter(c,tp)`
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_RAISE_MOON) and not c:IsCode(id) and not s.name_list[tp][c:GetCode()]
    let code = card::get_code(d, c)[0];
    card::is_set_card(d, c, &[SET_RAISE_MOON as u16]) && !card::is_code(d, c, &[ID]) && !name_listed(d, tp, code)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>1 and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 1
            && duel::is_existing_matching_card(d, Some(&|d, c| plfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>1 then
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
        // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, c| plfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if sc then
        if let Some(sc) = group::get_first(d, &g) {
            // Duel.ShuffleDeck(tp)
            duel::shuffle_deck(d, tp);
            // Duel.MoveToDeckTop(sc)
            duel::move_to_deck_top(d, sc);
            // Duel.ConfirmDecktop(tp,1)
            let _ = duel::confirm_decktop(d, tp, 1);
            // s.name_list[tp][sc:GetCode()]=true
            let code = card::get_code(d, sc)[0];
            let mut list = name_list(d, tp);
            list.push(Value::Int(code as i64));
            set_name_list(d, tp, list);
        }
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_TO_HAND)
    effect::set_code(d, e1, EFFECT_CANNOT_TO_HAND);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // e1:SetTarget(function(e,c,tp,re) return c:IsLocation(LOCATION_DECK) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_DECK))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.drconfilter(c)`
fn drconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsSetCard(SET_RAISE_MOON) and c:IsXyzMonster() and c:IsFaceup()
    card::is_race(d, c, RACE_SPELLCASTER) && card::is_set_card(d, c, &[SET_RAISE_MOON as u16]) && card::is_xyz_monster(d, c) && card::is_faceup(d, c)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local option_1=Duel.IsPlayerCanDraw(tp,1) and Duel.IsPlayerCanDraw(1-tp,1)
    let option_1 = duel::is_player_can_draw(d, tp, 1) && duel::is_player_can_draw(d, 1 - tp, 1);
    // local option_2=Duel.IsPlayerCanDraw(tp,1) and Duel.IsExistingMatchingCard(s.drconfilter,tp,LOCATION_MZONE,0,1,nil)
    let option_2 = duel::is_player_can_draw(d, tp, 1) && duel::is_existing_matching_card(d, Some(&drconfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return option_1 or option_2 end
    if a.chk == Some(0) {
        return option_1 || option_2;
    }
    // local choice=Duel.SelectEffect(tp, {option_1,aux.Stringid(id,4)}, {option_2,aux.Stringid(id,5)})
    let choice = duel::select_effect(d, tp, &[(option_1, aux::stringid(ID, 4)), (option_2, aux::stringid(ID, 5))]);
    // e:GetChainData().choice=choice
    chain::set_effect_data(d, a.e, None, chain::field::CARD, choice_value(choice));
    // local player=choice==1 and PLAYER_ALL or tp
    let player: u8 = if choice == Some(1) { PLAYER_ALL as u8 } else { tp };
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,player,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, player, 1);
    true
}
/// Stores the chosen option in the effect's chain data (`e:GetChainData().choice`)
fn choice_value(choice: Option<usize>) -> Value {
    choice.map_or(Value::Nil, |x| Value::Int(x as i64))
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local choice=e:GetChainData().choice
    let choice = chain::effect_data(d, a.e, None, chain::field::CARD).int();
    // if choice==1 then
    if choice == 1 {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
        // Duel.Draw(1-tp,1,REASON_EFFECT)
        duel::draw(d, 1 - a.tp, 1, REASON_EFFECT);
    // elseif choice==2 then
    } else if choice == 2 {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
}
