//! Trial and Tribulation
use super::prelude::*;

pub const ID: u32 = 26285788;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_TOHAND+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_TOHAND + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Keep track of how many monsters have been Tributed each turn
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
        effect::set_type(d, ge1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
        // ge1:SetCode(EVENT_RELEASE)
        effect::set_code(d, ge1, EVENT_RELEASE);
        // ge1:SetOperation(s.addcount)
        effect::set_operation(d, ge1, Some(operation(addcount)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
// function s.addcount(e,tp,eg,ep,ev,re,r,rp)
fn addcount(d: &mut Duel, a: &EffArgs) {
    // local p=nil
    // for tc in eg:Iter() do
    let eg = a.eg();
    for tc in group::iter(d, &eg) {
        // p=tc:GetReasonPlayer()
        let p = card::get_reason_player(d, tc);
        // if tc:IsMonster() and not tc:IsType(TYPE_TOKEN) and (tc:IsPreviousLocation(LOCATION_MZONE)
        //   or (tc:IsPreviousLocation(LOCATION_HAND) and tc:IsPreviousControler(p))) then
        if card::is_monster(d, tc)
            && !card::is_type(d, tc, TYPE_TOKEN)
            && (card::is_previous_location(d, tc, LOCATION_MZONE)
                || (card::is_previous_location(d, tc, LOCATION_HAND) && card::is_previous_controler(d, tc, p)))
        {
            // Duel.RegisterFlagEffect(p,id,RESET_PHASE|PHASE_END,0,1)
            duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:IsHasType(EFFECT_TYPE_ACTIVATE) end
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,PLAYER_ALL,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let handler = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, handler);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(_,tp) return Duel.GetFlagEffect(tp,id)>0 end)
    effect::set_condition(d, e1, Some(lua_fn(end_cond)));
    // e1:SetOperation(s.effectop)
    effect::set_operation(d, e1, Some(lua_fn(effectop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// function(_,tp) return Duel.GetFlagEffect(tp,id)>0 end
fn end_cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.int(1) as u8;
    duel::get_flag_effect(d, tp, ID) > 0
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.effectop(e,tp,eg,ep,ev,re,r,rp)
fn effectop(d: &mut Duel, a: Args) {
    let tp = a.int(1) as u8;
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // local ct=Duel.GetFlagEffect(tp,id)
    let ct = duel::get_flag_effect(d, tp, ID);
    // if ct==1 then
    if ct == 1 {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    // elseif ct==2 then
    } else if ct == 2 {
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| thfilter(d, c))), tp, LOCATION_GRAVE, 0, NO_EX);
        // if #g>=2 then
        if g.len() >= 2 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local tg=g:Select(tp,2,2,nil)
            let tg = group::select(d, &g, tp, 2, 2, false, NO_EX).nil();
            // Duel.SendtoHand(tg,nil,REASON_EFFECT)
            duel::send_to_hand(d, tg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,tg)
            duel::confirm_cards(d, 1 - tp, tg);
        }
    // else
    } else {
        // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // local tg=g:Select(tp,1,3,nil)
            let tg = group::select(d, &g, tp, 1, 3, false, NO_EX).nil();
            // if #tg==0 then return end
            if tg.is_empty() {
                return;
            }
            // Duel.HintSelection(tg,true)
            duel::hint_selection(d, tg.clone(), true);
            // Duel.Destroy(tg,REASON_EFFECT)
            duel::destroy(d, tg, REASON_EFFECT, None, None);
        }
    }
}
