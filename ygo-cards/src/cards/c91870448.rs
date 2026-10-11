//! Psychic Megacyber
use super::prelude::*;

pub const ID: u32 = 91870448;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_JINZO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card (from your hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place the opponent's Effect Monster that this card attacked face-up in their Spell & Trap Zone as a Continuous Trap
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_JINZO),tp,LOCATION_ONFIELD,0,1,nil) end)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.pltg)
    effect::set_target(d, e2, Some(cost_target(pltg)));
    // e2:SetOperation(s.plop)
    effect::set_operation(d, e2, Some(operation(plop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// The condition of `e2` (`function(e,tp)`).
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_JINZO),tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_JINZO]))),
        a.tp,
        LOCATION_ONFIELD,
        0,
        1,
        NO_EX,
    )
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        // and Duel.GetMatchingGroupCount(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)>Duel.GetMatchingGroupCount(Card.IsSpellTrap,tp,LOCATION_ONFIELD,0,nil)
        && duel::get_matching_group_count(d, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, NO_EX)
            > duel::get_matching_group_count(d, Some(&card::is_spell_trap), tp, LOCATION_ONFIELD, 0, NO_EX)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=Duel.GetAttackTarget()
    let dc = duel::get_attack_target(d);
    // if chk==0 then return Duel.GetLocationCount(1-tp,LOCATION_SZONE,tp)>0
    if a.chk == Some(0) {
        if duel::get_location_count(d, 1 - a.tp, LOCATION_SZONE as u8, Some(a.tp), None, None).0 <= 0 {
            return false;
        }
        // and d and d:IsControler(1-tp) and d:IsFaceup() and d:IsType(TYPE_EFFECT) end
        let Some(x) = dc else { return false };
        return card::is_controler(d, x, 1 - a.tp) && card::is_faceup(d, x) && card::is_type(d, x, TYPE_EFFECT);
    }
    // d:CreateEffectRelation(e)
    card::create_effect_relation(d, dc.nil(), a.e);
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    // local d=Duel.GetAttackTarget()
    let dc = duel::get_attack_target(d).nil();
    // if d:IsRelateToBattle() and d:IsControler(1-tp) and d:IsRelateToEffect(e) and not d:IsImmuneToEffect(e) then
    if card::is_relate_to_battle(d, dc) && card::is_controler(d, dc, 1 - a.tp) && card::is_relate_to_effect(d, dc, a.e) && !card::is_immune_to_effect(d, dc, a.e) {
        // if Duel.GetLocationCount(1-tp,LOCATION_SZONE)==0 then
        if duel::get_location_count(d, 1 - a.tp, LOCATION_SZONE as u8, None, None, None).0 == 0 {
            // Duel.SendtoGrave(d,REASON_RULE,nil,PLAYER_NONE)
            duel::send_to_grave(d, dc, REASON_RULE, None, Some(PLAYER_NONE as u8));
        // elseif Duel.MoveToField(d,tp,1-tp,LOCATION_SZONE,POS_FACEUP,d:IsMonsterCard()) then
        } else {
            let monster = card::is_monster_card(d, dc);
            if duel::move_to_field(d, dc, a.tp, 1 - a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, monster, None) {
                // Treated as a Continuous Trap
                // local e1=Effect.CreateEffect(e:GetHandler())
                let e1 = effect::create_effect(d, effect::get_handler(d, a.e).nil());
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_CHANGE_TYPE)
                effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
                // e1:SetValue(TYPE_TRAP|TYPE_CONTINUOUS)
                effect::set_value(d, e1, (TYPE_TRAP | TYPE_CONTINUOUS) as i64);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
                effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
                // d:RegisterEffect(e1)
                card::register_effect(d, dc, e1, false);
            }
        }
    }
}
