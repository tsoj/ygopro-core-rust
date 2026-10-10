//! Phantom of Yubel
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 80453041;

// s.material_setcode=SET_YUBEL
// s.listed_series={SET_YUBEL}
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_YUBEL],
    material_setcode: &[SET_YUBEL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials and Special Summon Procedure
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_YUBEL),s.fiendfilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_YUBEL as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| fiendfilter(d, c))),
        ],
    );
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,true)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(None),
            ..Default::default()
        },
    );
    // Cannot be used as Fusion Material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_FUSION_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_FUSION_MATERIAL);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by battle
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // You take no battle damage from battles involving this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_AVOID_BATTLE_DAMAGE);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Change a monster effect activated by the opponent
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and re:IsMonsterEffect() end)
    effect::set_condition(d, e4, Some(condition(chng_cond)));
    // e4:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e4, Some(cost::self_tribute()));
    // e4:SetTarget(s.chngtg)
    effect::set_target(d, e4, Some(cost_target(chngtg)));
    // e4:SetOperation(s.chngop)
    effect::set_operation(d, e4, Some(operation(chngop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.fiendfilter(c)`
fn fiendfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FIEND) and c:IsAttack(0) and c:IsDefense(0)
    card::is_race(d, c, RACE_FIEND) && card::is_attack(d, c, &[0]) && card::is_defense(d, c, &[0])
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(Card.IsAbleToDeckOrExtraAsCost,tp,LOCATION_MZONE|LOCATION_HAND|LOCATION_GRAVE,0,nil)
    duel::get_matching_group(
        d,
        Some(&|d, c| card::is_able_to_deck_or_extra_as_cost(d, c)),
        tp,
        LOCATION_MZONE | LOCATION_HAND | LOCATION_GRAVE,
        0,
        NO_EX,
    )
}
/// `s.contactop(g,tp)`
fn contactop(d: &mut Duel, g: &CardSet, tp: u8, _c: CardId) {
    // local fu,fd=g:Split(Card.IsFaceup,nil)
    let (fu, fd) = group::split(d, g, &|d, c| card::is_faceup(d, c), NO_EX);
    // if #fu>0 then Duel.HintSelection(fu,true) end
    if !fu.is_empty() {
        duel::hint_selection(d, fu, true);
    }
    // if #fd>0 then Duel.ConfirmCards(1-tp,fd) end
    if !fd.is_empty() {
        duel::confirm_cards(d, 1 - tp, &fd);
    }
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST|REASON_MATERIAL)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_COST | REASON_MATERIAL, None);
}
/// `s.yubelfilter(c)`
fn yubelfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_YUBEL) and c:IsMonster() and (c:IsFaceup() or not c:IsOnField())
    card::is_set_card(d, c, &[SET_YUBEL as u16]) && card::is_monster(d, c) && (card::is_faceup(d, c) || !card::is_on_field(d, c))
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and re:IsMonsterEffect() end`
fn chng_cond(d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp && effect::is_monster_effect(d, a.re.nil())
}
/// `s.chngtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn chngtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.yubelfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_DECK,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&yubelfilter), a.tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_DECK, 0, 1, c.into());
    }
    true
}
/// `s.chngop(e,tp,eg,ep,ev,re,r,rp)`
fn chngop(d: &mut Duel, a: &EffArgs) {
    // local g=Group.CreateGroup()
    let g = CardSet::new();
    // Duel.ChangeTargetCard(ev,g)
    duel::change_target_card(d, a.ev as u8, &g);
    // Duel.ChangeChainOperation(ev,s.repop)
    duel::change_chain_operation(d, a.ev as u8, operation(repop));
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(1-tp,s.yubelfilter,1-tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, 1 - tp, Some(&yubelfilter), 1 - tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT,LOCATION_GRAVE,1-tp)
        duel::destroy(d, g, REASON_EFFECT, Some(LOCATION_GRAVE), Some(1 - tp));
    }
}
