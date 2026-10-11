//! Readying of Rites
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 71771004;
const TOKEN_SACIFICIAL_OFFERING: u32 = ID + 1;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_SACIFICIAL_OFFERING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 Fiend Ritual Monster from your Deck to your hand, then if you added a Pendulum Monster, you can Special Summon 1 "Sacrificial Offering Token"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.drcon)
    effect::set_condition(d, e2, Some(condition(drcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FIEND) and c:IsRitualMonster() and c:IsAbleToHand()
    card::is_race(d, c, RACE_FIEND) && card::is_ritual_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOKEN, None, 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// `TOKEN_SACIFICIAL_OFFERING`: Fiend/DARK/Level 1/ATK 300/DEF 300 token
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(300),
        defense: Some(300),
        level: Some(1),
        race: Some(RACE_FIEND),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND) then
    let Some(sc) = sc else { return };
    if duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_HAND) {
        // Duel.ConfirmCards(1-tp,sc)
        duel::confirm_cards(d, 1 - tp, sc);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // if sc:IsType(TYPE_PENDULUM) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_SACIFICIAL_OFFERING,0,TYPES_TOKEN,300,300,1,RACE_FIEND,ATTRIBUTE_DARK)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if card::is_type(d, sc, TYPE_PENDULUM)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, TOKEN_SACIFICIAL_OFFERING, token_data(), None, None, 0)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            // local token=Duel.CreateToken(tp,TOKEN_SACIFICIAL_OFFERING)
            let token = duel::create_token(d, tp, TOKEN_SACIFICIAL_OFFERING).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)>0 then
            if duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                // token:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,3))
                card::register_flag_effect(d, token, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 3));
                // While you control that Token, you cannot Special Summon from the Extra Deck
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetType(EFFECT_TYPE_FIELD)
                effect::set_type(d, e1, EFFECT_TYPE_FIELD);
                // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
                effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
                // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
                effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
                // e1:SetRange(LOCATION_MZONE)
                effect::set_range(d, e1, LOCATION_MZONE);
                // e1:SetAbsoluteRange(tp,1,0)
                effect::set_absolute_range(d, e1, tp, 1, 0);
                // e1:SetCondition(function(e) return e:GetHandler():IsControler(e:GetOwnerPlayer()) end)
                effect::set_condition(
                    d,
                    e1,
                    Some(condition(|d, a| {
                        let h = effect::get_handler(d, a.e).nil();
                        let owner = effect::get_owner_player(d, a.e);
                        card::is_controler(d, h, owner)
                    })),
                );
                // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) end)
                effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA))));
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // token:RegisterEffect(e1,true)
                card::register_effect(d, token, e1, true);
            }
        }
    }
}
// function s.drconfilter(c,tp)
fn drconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRitualMonster() and c:IsControler(tp)
    card::is_ritual_monster(d, c) && card::is_controler(d, c, tp)
}
// function s.drcon(e,tp,eg,ep,ev,re,r,rp)
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return ep==1-tp and eg:IsExists(s.drconfilter,1,nil,tp)
    if a.ep != 1 - tp {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| drconfilter(d, c, tp), 1, NO_EX)
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
