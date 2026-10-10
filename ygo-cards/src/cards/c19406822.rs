//! Kotodama
use super::prelude::*;

pub const ID: u32 = 19406822;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // adjust
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_ADJUST);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_operation(d, e1, Some(operation(adjustop)));
    card::register_effect(d, c, e1, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e1:SetLabelObject(g)
    let g = CardSet::new();
    effect::set_label_object(d, e1, Value::Group(g));
}
/// `function s.filter(c,g,pg)`
fn filter(d: &mut Duel, c: CardId, g: &CardSet, pg: &CardSet) -> bool {
    // if pg:IsContains(c) then return false end
    if pg.contains(c) {
        return false;
    }
    // local code=c:GetCode()
    let code = card::get_code(d, c).first().copied().nil();
    // return g:IsExists(Card.IsCode,1,c,code) or pg:IsExists(Card.IsCode,1,c,code)
    group::is_exists(d, g, &|d, cc| card::is_code(d, cc, &[code]), 1, c.into())
        || group::is_exists(d, pg, &|d, cc| card::is_code(d, cc, &[code]), 1, c.into())
}
fn adjustop(d: &mut Duel, a: &EffArgs) {
    // local phase=Duel.GetCurrentPhase()
    let phase = duel::get_current_phase(d);
    // if (phase==PHASE_DAMAGE and not Duel.IsDamageCalculated()) or phase==PHASE_DAMAGE_CAL then return end
    if (phase == PHASE_DAMAGE && !duel::is_damage_calculated(d)) || phase == PHASE_DAMAGE_CAL {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local pg=e:GetLabelObject()
    // (a group label object is a value: it is written back after each change)
    let mut pg = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // if c:GetFlagEffect(id)==0 then
    if card::get_flag_effect(d, c, ID) == 0 {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_DISABLE,0,1)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 1, 0, 0);
        // pg:Clear()
        pg.clear();
        set_pg(d, a.e, &pg);
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,0,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local dg=g:Filter(s.filter,nil,g,e:GetLabelObject())
    let dg = group::filter(d, &g, &|d, c| filter(d, c, &g, &pg), NO_EX);
    // if #dg==0 or Duel.Destroy(dg,REASON_EFFECT)==0 then
    if dg.is_empty() || duel::destroy(d, dg.clone(), REASON_EFFECT, None, None) == 0 {
        // pg:Clear()
        pg.clear();
        // pg:Merge(g)
        group::add_card(&mut pg, &g);
        // pg:Sub(dg)
        group::remove_card(&mut pg, &dg);
        set_pg(d, a.e, &pg);
    } else {
        // g=Duel.GetMatchingGroup(Card.IsFaceup,0,LOCATION_MZONE,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        // pg:Clear()
        pg.clear();
        // pg:Merge(g)
        group::add_card(&mut pg, &g);
        // pg:Sub(dg)
        group::remove_card(&mut pg, &dg);
        set_pg(d, a.e, &pg);
        // Duel.Readjust()
        duel::readjust(d);
    }
}
/// Stores the group in the label object of the effect (the Lua group is changed in place).
fn set_pg(d: &mut Duel, e: EffectId, pg: &CardSet) {
    effect::set_label_object(d, e, Value::Group(pg.clone()));
}
